#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32h7xx_hal::{pac, prelude::*};

use freertos_rust::*;
use anychain_core::{hex, libsecp256k1, amount};
// use anychain_core::{amount};

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World!");

    let public_key = "546974616e2028746974616e6275696c6465722e78797a29";
    let public_key = hex::decode(public_key).unwrap();
    rprintln!("{:?}", public_key);

    let s = amount::to_basic_unit("0.0001037910", 7);
    rprintln!("{:?}", s);

    loop {
        // 延时500ms
    }
}

// #[alloc_error_handler]
// fn alloc_error(_layout: Layout) -> ! {
//     //set_led(true);
//     asm::bkpt();
//     loop {}
// }

#[no_mangle]
fn vApplicationStackOverflowHook(pxTask: FreeRtosTaskHandle, pcTaskName: FreeRtosCharPtr) {
    asm::bkpt();
}
