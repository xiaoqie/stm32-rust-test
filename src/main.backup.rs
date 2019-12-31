//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manaual for an explanation. This is not an issue on the blue pill.

//#![deny(unsafe_code)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(concat_idents)]

#[cfg(not(feature = "__my_building__"))]
include!("rtfm-expansion-helper.rs");

//mod stm32_hal_macro;
//mod stm32f103_hal;

// Plug in the allocator crate
extern crate alloc;
extern crate alloc_cortex_m;
extern crate panic_halt;

mod stm32f103_hal;

use cortex_m_semihosting::hprintln;
use rtfm::cyccnt::{Instant, U32Ext as _};
use stm32f1::stm32f103;

use alloc::string::String;
use alloc::vec::Vec;

use cortex_m::asm;

#[rtfm::app(device = stm32f103, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        device: stm32f103::Peripherals,
        #[init(Vec::new())]
        v: Vec<i32>,
        #[init(String::new())]
        usart1_data: String,
    }
    /// cortex-m-rtfm takes care of NVIC, DCB.enable_trace()
    #[init(spawn = [main_loop], schedule = [main_loop])]
    fn init(cx: init::Context) -> init::LateResources {
        stm32f103_hal::init_allocator();

        let p = stm32f103::Peripherals::take().unwrap();
        let mut cp = stm32f103::CorePeripherals::take().unwrap();

        stm32f103_hal::init_clock(&p);

        cp.DWT.enable_cycle_counter();

        p.RCC.apb2enr.modify(|_r, w| w.iopben().enabled());
        p.RCC.apb2enr.modify(|_r, w| w.iopeen().enabled());
        p.RCC.apb2enr.modify(|_r, w| w.afioen().enabled());

        p.RCC.apb2enr.modify(|_r, w| w.usart1en().enabled());
        //        p.RCC.apb2rstr.modify(|_r, w| w.usart1rst().set_bit());
        //        p.RCC.apb2rstr.modify(|_r, w| w.usart1rst().reset());

        p.USART1.brr.write(|w| w.div_fraction().bits(1).div_mantissa().bits(39));
        p.USART1.cr1.modify(|_r, w| w.te().enabled().re().enabled().ue().enabled());
        p.USART1.cr1.modify(|_r, w| w.rxneie().enabled());
        p.RCC.apb2enr.modify(|_r, w| w.iopaen().enabled());
        p.GPIOA.crh.modify(|_r, w| w.cnf9().alt_push_pull().mode9().output50());
        p.GPIOA.crh.modify(|_r, w| w.cnf10().alt_push_pull().mode10().input());

        p.RCC.apb1enr.modify(|_r, w| w.tim3en().enabled());

        p.GPIOB.crl.modify(|_r, w| w.mode5().output50().cnf5().push_pull());
        p.GPIOE.crl.modify(|_r, w| w.mode5().output50().cnf5().push_pull());
        p.GPIOE.crl.modify(|_r, w| w.mode3().input().cnf3().alt_push_pull());
        p.GPIOE.crl.modify(|_r, w| w.mode4().input().cnf4().alt_push_pull());

        p.GPIOB.bsrr.write(|w| w.br5().reset());
        p.GPIOE.bsrr.write(|w| w.br5().reset());

        p.GPIOE.bsrr.write(|w| w.bs3().set());
        p.GPIOE.bsrr.write(|w| w.bs4().set());

        p.AFIO.exticr1.write(|w| unsafe { w.exti3().bits(0b0100) });
        p.EXTI.imr.write(|w| w.mr3().set_bit());
        p.EXTI.ftsr.write(|w| w.tr3().set_bit());
        p.EXTI.rtsr.write(|w| w.tr3().set_bit());

        p.TIM3.dier.write(|w| w.uie().enabled());
        p.TIM3.cr1.write(|w| w.cen().enabled());
        p.TIM3.arr.write(|w| w.arr().bits(4999));
        p.TIM3.psc.write(|w| w.psc().bits(7199));

        //let now = c.start;
        //c.schedule.main_loop(now + (102_000_000).cycles()).unwrap();
        cx.spawn.main_loop().unwrap();

        init::LateResources { device: p }
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(priority = 1, resources = [device, v, usart1_data], schedule = [main_loop])]
    fn main_loop(cx: main_loop::Context) {
        cx.schedule.main_loop(cx.scheduled + (72_000_000).cycles()).unwrap();

        //        let p: &mut stm32f103::Peripherals = cx.resources.device;
        let p = unsafe { stm32f103::Peripherals::steal() };

        let on = !p.GPIOE.idr.read().idr5().bits();
        if on {
            p.GPIOE.bsrr.write(|w| w.bs5().set());
        } else {
            p.GPIOE.bsrr.write(|w| w.br5().reset());
        }
        let v: &mut Vec<i32> = cx.resources.v;
        //        hprintln!("{:?}, {:?}", cx.scheduled, Instant::now()).unwrap();
        //        hprintln!("{:?}", v).unwrap();
        //        hprintln!("{}", cx.resources.usart1_data).unwrap();
        for c in cx.resources.usart1_data.chars() {
            p.USART1.dr.write(|w| w.dr().bits(c as u16));
            while p.USART1.sr.read().txe().bit_is_clear() {}
        }
        cx.resources.usart1_data.clear();
    }

    #[task(binds = EXTI3, resources = [device, v], schedule = [main_loop])]
    fn exti3(cx: exti3::Context) {
        asm::delay(72 * 1000);
        //        let p: &mut stm32f103::Peripherals = cx.resources.device;
        let p = unsafe { stm32f103::Peripherals::steal() };
        let on = !p.GPIOB.idr.read().idr5().bits();
        if on {
            p.GPIOB.bsrr.write(|w| w.bs5().set());
        } else {
            p.GPIOB.bsrr.write(|w| w.br5().reset());
        }
        let v: &mut Vec<i32> = cx.resources.v;
        v.push(1);
        p.EXTI.pr.write(|w| w.pr3().set_bit());
    }

    #[task(binds = USART1, resources = [usart1_data])]
    fn usart1(cx: usart1::Context) {
        let p = unsafe { stm32f103::Peripherals::steal() };
        if p.USART1.sr.read().rxne().bit_is_set() {
            let c = p.USART1.dr.read().dr().bits() as u8 as char;
            //            let str: &mut String = cx.resources.usart1_data;
            cx.resources.usart1_data.push(c);
            //            p.USART1.dr.write(|w| w.dr().bits(c as u16));
            //            while p.USART1.sr.read().txe().bit_is_clear() {}
        }
    }

    #[task(binds = TIM3, resources = [device], schedule = [main_loop])]
    fn tim3(cx: tim3::Context) {
        //        let p: &mut stm32f103::Peripherals = cx.resources.device;
        let p = unsafe { stm32f103::Peripherals::steal() };
        if p.TIM3.sr.read().uif().bit() {
            let on = !p.GPIOB.idr.read().idr5().bits();
            if on {
                p.GPIOB.bsrr.write(|w| w.bs5().set());
            } else {
                p.GPIOB.bsrr.write(|w| w.br5().reset());
            }
        }
        //        cx.schedule.main_loop(Instant::now() + (1_000_000).cycles()).unwrap();
        p.TIM3.sr.modify(|_r, w| w.uif().clear());
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USB_LP_CAN_RX0();
    }
};
