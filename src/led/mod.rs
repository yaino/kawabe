use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

/// LED(プルアップ接続)
pub struct Led {
    pub pin: OutputPin,
}

impl Led {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pin: Gpio::new()?.get(pin)?.into_output(),
        })
    }

    pub fn light_on(&mut self) {
        self.pin.set_high()
    }

    pub fn light_off(&mut self) {
        self.pin.set_low()
    }
}
