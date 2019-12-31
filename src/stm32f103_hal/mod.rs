use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m::asm;
pub use stm32f1::stm32f103::Peripherals;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}

pub fn init_allocator() {
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024 * 32; // in bytes
    unsafe { ALLOCATOR.init(start, size) }
}

pub fn init_clock(p: &Peripherals) {
    p.RCC.cr.write(|w| w.hseon().on());
    while !p.RCC.cr.read().hserdy().bit() {}
    p.RCC.cfgr.write(|w| w.pllsrc().hse_div_prediv().pllmul().mul9().ppre1().div2().ppre2().div1());
    p.FLASH.acr.write(|w| w.latency().ws2().prftbe().set_bit());
    p.RCC.cr.modify(|_r, w| w.pllon().on());
    while !p.RCC.cr.read().pllrdy().bit() {}
    p.RCC.cfgr.modify(|_r, w| w.sw().pll());
    while !(p.RCC.cfgr.read().sws().bits() == 2) {}
}

#[macro_use]
mod gpio;
pub use gpio::*;
mod pin_definitions;
pub use pin_definitions::*;

mod exti;
pub use exti::*;
