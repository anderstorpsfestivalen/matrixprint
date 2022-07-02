use crate::error::Error;
use log::{debug, info};
use rppal::gpio::{Gpio, OutputPin};
use tokio::time::Duration;

pub struct Light {
    relay: OutputPin,
}

impl Light {
    pub async fn init(pin: u8) -> Result<Light, Error> {
        info!("Initalizing relay on pin {}", pin);
        let mut rl = Gpio::new()?.get(pin)?.into_output();
        rl.set_low();
        Ok(Light { relay: rl })
    }

    pub async fn alert(&mut self, r: Duration) {
        debug!("opening relay");
        self.relay.set_high();
        tokio::time::sleep(r).await;
        debug!("closing relay");
        self.relay.set_low();
    }
}
