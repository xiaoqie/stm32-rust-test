use crate::stm32f103_hal::EXTILine::{Line3, NoInterrupt};
use core::marker::PhantomData;
use stm32f1::stm32f103::Peripherals;

pub struct Inactive;
pub struct Unset;

pub struct A;
pub struct B;
pub struct C;
pub struct D;
pub struct E;
pub struct F;
pub struct G;

pub struct EXTI3<SOURCE> {
    _source: PhantomData<SOURCE>,
}

impl<SOURCE> EXTI3<SOURCE> {
    pub fn select_gpioe(self) -> EXTI3<E> {
        let p = unsafe { Peripherals::steal() };
        p.AFIO.exticr1.write(|w| unsafe { w.exti3().bits(0b0100) });
        EXTI3 { _source: PhantomData }
    }

    pub fn enable_rising_trigger(&mut self) {
        let p = unsafe { Peripherals::steal() };
        p.EXTI.rtsr.write(|w| w.tr3().set_bit());
    }

    pub fn enable_falling_trigger(&mut self) {
        let p = unsafe { Peripherals::steal() };
        p.EXTI.ftsr.write(|w| w.tr3().set_bit());
    }
}

impl EXTI3<Inactive> {
    pub fn take() -> EXTI3<Unset> {
        let p = unsafe { Peripherals::steal() };
        p.RCC.apb2enr.modify(|_r, w| w.afioen().enabled());
        p.EXTI.imr.write(|w| w.mr3().set_bit());
        EXTI3 { _source: PhantomData }
    }
}

pub enum EXTILine {
    Line0,
    Line1,
    Line2,
    Line3,
    Line4,
    Line5,
    Line6,
    Line7,
    NoInterrupt,
}

pub fn check_exti_source() -> EXTILine {
    let p = unsafe { Peripherals::steal() };
    let a = p.EXTI.pr.read();
    if a.pr3().bit_is_set() {
        p.EXTI.pr.write(|w| w.pr3().set_bit());
        Line3
    } else {
        NoInterrupt
    }
}
