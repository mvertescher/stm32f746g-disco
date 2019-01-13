//! Board Support Crate for the STM32F746G-Discovery

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate stm32f7xx_hal as hal;

pub mod led;

/// Create the default user LED.
#[macro_export]
macro_rules! user_led {
    ($p:ident) => {
        {
            let gpioi = $p.GPIOI.split();
            stm32f746g_disco::led::Led::new(gpioi)
        }
    }
}
