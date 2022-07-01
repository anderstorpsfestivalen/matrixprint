use crate::error::Error;
use rppal::gpio::{Gpio, OutputPin};

const RELAY: u8 = 23;

pub struct Light {
    relay: OutputPin,
}

impl Light {
    pub async fn create() -> Result<Light, Error> {
        let mut rl = Gpio::new()?.get(RELAY)?.into_output();
        Ok(())
    }
}
