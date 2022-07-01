mod conn;
mod error;
mod light;
mod message;
mod printer;
use pretty_env_logger;
use std::env;

use anyhow::{Context, Result};

#[tokio::main]

async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Open printer
    let mut mp = printer::Printer::open("/dev/usb/lp0")
        .await
        .context("Printer")?;

    // Initialize saftblandare

    let mut sf = light::Light::init(23).await.context("GPIO")?;

    let (mut c, mut rx) = conn::Connection::new("wss://mch.anderstorpsfestivalen.se/kernel/pipe")
        .await
        .context("Websocket")?;

    c.connect().await.unwrap();

    while let Some(i) = rx.recv().await {
        dbg!(&i);
        mp.print(i).await;
    }

    Ok(())
}
