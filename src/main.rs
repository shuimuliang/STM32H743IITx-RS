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

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World!");

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
