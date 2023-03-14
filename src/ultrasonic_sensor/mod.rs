use std::{
    error::Error,
    sync::mpsc::Sender,
    thread,
    time::{Duration, SystemTime},
};

use rppal::gpio::{Gpio, InputPin, OutputPin};

pub struct UltrasonicSensor {
    trigger: OutputPin,
    echo: InputPin,
}

impl UltrasonicSensor {
    pub fn new(trigger_pin: u8, echo_pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            trigger: Gpio::new()?.get(trigger_pin)?.into_output(),
            echo: Gpio::new()?.get(echo_pin)?.into_input(),
        })
    }
    pub fn run(mut self, tx: Sender<f32>) {
        thread::spawn(move || loop {
            self.trigger.set_high();
            thread::sleep(Duration::from_nanos(10_u64));
            self.trigger.set_low();

            let mut start_time = SystemTime::now();
            while self.echo.is_low() {
                start_time = SystemTime::now();
            }

            let mut elapsed_time = start_time.elapsed().unwrap();
            while self.echo.is_high() {
                elapsed_time = start_time.elapsed().unwrap();
            }

            let distanse = elapsed_time.as_micros() as f32 * 0.0343 / 2.0;

            if let Err(e) = tx.send(distanse) {
                println!("{e:?}")
            }
            thread::sleep(Duration::from_millis(250));
        });
    }
}
