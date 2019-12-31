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

// Plug in the allocator crate
extern crate alloc;
extern crate alloc_cortex_m;
extern crate panic_halt;

mod elite_board;
mod stm32f103_hal;

use cortex_m_semihosting::hprintln;
use elite_board::*;
use rtfm::cyccnt::{Instant, U32Ext as _};
use stm32f1::stm32f103;
use stm32f103_hal::*;

use alloc::string::String;
use alloc::vec::Vec;

use core::marker::PhantomData;
use cortex_m::asm;
use rtfm::Mutex;
use stm32f1::stm32f103::Interrupt::EXTI3;

#[cfg(not(feature = "__my_building__"))]
include!("rtfm-autocompletion-helper.rs");

#[rtfm::app(device = stm32f103, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led0: LED0,
        led1: LED1,
        beeper: Beeper,
        key0: Key0,
        key1: Key1,
        key_up: KeyUp,
        #[init(Vec::new())]
        list: Vec<i32>,
        #[init(String::new())]
        usart1_data: String,
    }
    /// cortex-m-rtfm takes care of NVIC, DCB.enable_trace()
    #[init(spawn = [main_loop])]
    fn init(cx: init::Context) -> init::LateResources {
        let p = stm32f103::Peripherals::take().unwrap();
        let mut cp = stm32f103::CorePeripherals::take().unwrap();
        cp.DWT.enable_cycle_counter();

        init_clock(&p);
        init_allocator();

        let mut led0 = LED0::new(PB5::take().into_push_pull_output());
        let mut led1 = LED1::new(PE5::take().into_push_pull_output());
        let mut beeper = Beeper::new(PB8::take().into_push_pull_output());
        let key0 = Key0::new(PE4::take().into_pull_up_input());
        let key1 = Key1::new(PE3::take().into_pull_up_input());
        let key_up = KeyUp::new(PA0::take().into_pull_down_input());

        let mut exti3 = stm32f103_hal::EXTI3::take().select_gpioe();
        //        exti3.enable_falling_trigger();
        exti3.enable_rising_trigger();

        led0.off();
        led1.on();
        beeper.off();

        cx.spawn.main_loop().unwrap();

        init::LateResources {
            led0,
            led1,
            beeper,
            key0,
            key1,
            key_up,
        }
    }

    #[idle(resources = [led0])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(resources = [led0, led1, beeper, key0, key1, key_up], schedule = [main_loop])]
    fn main_loop(cx: main_loop::Context) {
        //        cx.resources.led0.toggle();
        //        cx.resources.led1.toggle();
        //        cx.resources.beeper.toggle();
        //        cx.resources.led0.set(cx.resources.key0.is_pressing());
        //        cx.resources.led1.set(cx.resources.key1.is_pressing());
        //        cx.resources.beeper.set(cx.resources.key_up.is_pressing());
        cx.schedule.main_loop(cx.scheduled + (72_000_000 / 10).cycles()).unwrap();
    }

    #[task(binds = EXTI3, resources = [led0], schedule = [main_loop])]
    fn exti3(cx: exti3::Context) {
        asm::delay(72 * 10000);
        match check_exti_source() {
            Line3 => {
                cx.resources.led0.toggle();
            }
            _ => {}
        }
    }
    /*
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
    */
    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USB_LP_CAN_RX0();
    }
};
