//! Blinks the user LED on the bottom of the board

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f746g_disco::{
    hal::{prelude::*, device, delay::Delay},
    user_led,
};

#[entry]
fn main() -> ! {
    let p = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut led = user_led!(p);

    loop {
        led.on();
        delay.delay_ms(1_000_u16);
        led.off();
        delay.delay_ms(1_000_u16);
    }
}
