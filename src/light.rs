use crate::error::Error;
use rppal::gpio::{Gpio, OutputPin};

pub struct Light {
    relay: OutputPin,
}

impl Light {
    pub async fn init(pin: u8) -> Result<Light, Error> {
        let mut rl = Gpio::new()?.get(pin)?.into_output();
        rl.set_low();
        Ok(Light { relay: rl })
    }

    pub async fn alert(&mut self, d: tokio::time::Duration) {
        self.relay.set_high();
        tokio::time::sleep(d).await;
        self.relay.set_low();
    }
}
