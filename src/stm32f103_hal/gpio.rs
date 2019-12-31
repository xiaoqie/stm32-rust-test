pub use core::marker::PhantomData;
pub use stm32f1::stm32f103::Peripherals;

pub struct Active;
pub struct Inactive;

pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}
pub trait InputPin {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}
pub struct Analog;
pub struct Floating;
pub struct PullUp;
pub struct PullDown;

pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}
pub trait OutputPin {
    fn set(&self, status: bool);
    fn is_set(&self) -> bool;
    fn toggle(&self);
}
pub struct PushPull;
pub struct OpenDrain;
pub struct AlternatePushPull;
pub struct AlternateOpenDrain;

macro_rules! gpio_pin {
    ($PXxx:ident, $crl_or_crh:ident, $GPIOX:ident, $iopXen:ident, $modex: ident, $cnfx:ident, $brx:ident, $bsx:ident, $idrx:ident, $odrx:ident) => {
        pub struct $PXxx<MODE = Inactive> {
            _mode: PhantomData<MODE>,
            device: Peripherals,
        }

        impl<MODE> $PXxx<MODE> {
            #[inline(always)]
            pub fn into_push_pull_output(self) -> $PXxx<Output<PushPull>> {
                self.device.$GPIOX.$crl_or_crh.modify(|_r, w| w.$modex().output50().$cnfx().push_pull());

                $PXxx {
                    _mode: PhantomData,
                    device: self.device,
                }
            }
            pub fn into_pull_up_input(self) -> $PXxx<Input<PullUp>> {
                self.device.$GPIOX.$crl_or_crh.modify(|_r, w| w.$modex().input().$cnfx().alt_push_pull());
                self.device.$GPIOX.odr.modify(|_r, w| w.$odrx().set_bit());

                $PXxx {
                    _mode: PhantomData,
                    device: self.device,
                }
            }
            pub fn into_pull_down_input(self) -> $PXxx<Input<PullDown>> {
                self.device.$GPIOX.$crl_or_crh.modify(|_r, w| w.$modex().input().$cnfx().alt_push_pull());
                self.device.$GPIOX.odr.modify(|_r, w| w.$odrx().clear_bit());

                $PXxx {
                    _mode: PhantomData,
                    device: self.device,
                }
            }
        }

        impl $PXxx<Inactive> {
            #[inline(always)]
            pub fn take() -> $PXxx<Active> {
                let p = unsafe { Peripherals::steal() };
                p.RCC.apb2enr.modify(|_r, w| w.$iopXen().enabled());

                $PXxx { _mode: PhantomData, device: p }
            }
        }

        impl<MODE> OutputPin for $PXxx<Output<MODE>> {
            #[inline(always)]
            fn set(&self, status: bool) {
                if status {
                    self.device.$GPIOX.bsrr.write(|w| w.$bsx().set());
                } else {
                    self.device.$GPIOX.bsrr.write(|w| w.$brx().reset());
                }
            }
            #[inline(always)]
            fn is_set(&self) -> bool {
                self.device.$GPIOX.idr.read().$idrx().bits()
            }
            #[inline(always)]
            fn toggle(&self) {
                self.set(!self.is_set());
            }
        }

        impl<MODE> InputPin for $PXxx<Input<MODE>> {
            #[inline(always)]
            fn is_high(&self) -> bool {
                self.device.$GPIOX.idr.read().$idrx().bits()
            }
            fn is_low(&self) -> bool {
                !self.is_high()
            }
        }
    };
}
