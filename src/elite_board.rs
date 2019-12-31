use crate::stm32f103_hal::*;

pub struct LowIsOnOutput<PIN> {
    pin: PIN,
}

impl<PIN> LowIsOnOutput<PIN>
where
    PIN: OutputPin,
{
    pub fn new(pin: PIN) -> LowIsOnOutput<PIN> {
        LowIsOnOutput { pin }
    }

    pub fn on(&mut self) {
        self.set(false);
    }

    pub fn off(&mut self) {
        self.set(false);
    }

    pub fn is_on(&self) -> bool {
        !self.pin.is_set()
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    pub fn set(&mut self, on: bool) {
        self.pin.set(!on);
    }
}

pub struct HighIsOnOutput<PIN> {
    pin: PIN,
}

impl<PIN> HighIsOnOutput<PIN>
where
    PIN: OutputPin,
{
    pub fn new(pin: PIN) -> HighIsOnOutput<PIN> {
        HighIsOnOutput { pin }
    }

    pub fn on(&mut self) {
        self.set(true);
    }

    pub fn off(&mut self) {
        self.set(false);
    }

    pub fn is_on(&self) -> bool {
        self.pin.is_set()
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    pub fn set(&mut self, on: bool) {
        self.pin.set(on);
    }
}
pub struct PullUpButton<PIN> {
    pin: PIN,
}
impl<PIN> PullUpButton<PIN>
where
    PIN: InputPin,
{
    pub fn new(pin: PIN) -> PullUpButton<PIN> {
        PullUpButton { pin }
    }
    pub fn is_pressing(&self) -> bool {
        self.pin.is_low()
    }
}
pub struct PullDownButton<PIN> {
    pin: PIN,
}
impl<PIN> PullDownButton<PIN>
where
    PIN: InputPin,
{
    pub fn new(pin: PIN) -> PullDownButton<PIN> {
        PullDownButton { pin }
    }
    pub fn is_pressing(&self) -> bool {
        self.pin.is_high()
    }
}

pub type LED0 = LowIsOnOutput<PB5<Output<PushPull>>>;
pub type LED1 = LowIsOnOutput<PE5<Output<PushPull>>>;
pub type Beeper = HighIsOnOutput<PB8<Output<PushPull>>>;
pub type Key0 = PullUpButton<PE4<Input<PullUp>>>;
pub type Key1 = PullUpButton<PE3<Input<PullUp>>>;
pub type KeyUp = PullDownButton<PA0<Input<PullDown>>>;
