//! On-board user LEDs

use crate::hal::prelude::*;

use crate::hal::gpio::gpioi::{self, PI, PI1};
use crate::hal::gpio::{Output, PushPull};

/// The lone user LED
pub type LD2 = PI1<Output<PushPull>>;

/// One of the on-board user LEDs
pub struct Led {
    pix: PI<Output<PushPull>>,
}

impl Into<Led> for LD2 {
    fn into(self) -> Led {
        Led {
            pix: self.downgrade(),
        }
    }
}

impl Led {
    /// Initialize the single user LED
    pub fn new(gpioi: gpioi::Parts) -> Self {
        gpioi.pi1.into_push_pull_output().into()
    }
}

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pix.set_low()
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pix.set_high()
    }
}
