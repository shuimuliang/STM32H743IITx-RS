#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // 获取cortex核心外设和stm32h7的所有外设
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Power 设置
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    // 初始化RCC
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(200.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // 设置LED对应的GPIO
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let mut led = gpioe.pe3.into_push_pull_output();

    // cortex-m已经实现好了delay函数，直接拿到，下面使用
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        // 点灯并且输出RTT日志
        led.toggle();
        rprintln!("Hello World!");
        // 延时500ms
        delay.delay_ms(500_u16);
    }
}
