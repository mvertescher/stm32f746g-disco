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

/// Configure the default serial port with the default configuration.
#[macro_export]
macro_rules! serial {
    ($p:ident, $clocks:ident) => {
        {
            let gpioa = $p.GPIOA.split();
            let gpiob = $p.GPIOB.split();

            let tx = gpioa.pa9.into_alternate_af7();
            let rx = gpiob.pb7.into_alternate_af7();

            stm32f746g_disco::hal::serial::Serial::usart1($p.USART1, (tx, rx), 115_200.bps(), $clocks)
        }
    };
}
