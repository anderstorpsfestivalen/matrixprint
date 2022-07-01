mod conn;
mod error;
mod light;
mod message;
mod printer;
use pretty_env_logger;
use std::env;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Open printer
    let mut matrixprinter = printer::Printer::open("/dev/usb/lp0").await?;

    // Initialize saftblandare
    let mut saftblandare = light::Light::init(23).await?;

    // Connect to backend
    let (mut c, mut rx) =
        conn::Connection::new("wss://mch.anderstorpsfestivalen.se/kernel/pipe").await?;
    c.connect().await.unwrap();

    // Forever ?
    while let Some(i) = rx.recv().await {
        // Spin saftblandare for 4 secs
        saftblandare
            .alert(tokio::time::Duration::from_secs(4))
            .await;

        // Send the message the the printer
        matrixprinter.print(i).await?;
    }

    Ok(())
}
