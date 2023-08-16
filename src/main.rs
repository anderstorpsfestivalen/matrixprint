mod cc;
mod cli;
mod conn;
mod error;
mod stats;

#[cfg(feature = "rpi")]
mod light;

mod message;
mod printer;
use anyhow::Result;
use clap::Parser;
use log::{info, warn};
use pretty_env_logger;
use std::env;
#[cfg(feature = "rpi")]
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let opts = cli::Args::parse();

    // Setup stats tracking
    let stats = stats::Stats::new(&opts.stats_url, &opts.stats_key);

    let stats = match stats {
        Ok(v) => Some(v),
        Err(e) => {
            warn!("{}, disabling stats tracking", e);
            None
        }
    };

    // Open printer
    let mut matrixprinter = printer::Printer::open(&opts.printer_path, stats).await?;

    // Initialize saftblandare
    #[cfg(feature = "rpi")]
    let mut saftblandare = light::Light::init(opts.relaypin).await?;

    // Connect to backend
    let (_c, mut rx) = conn::Connection::new(&opts.websocket).await?;

    // Forever ?
    while let Some(i) = rx.recv().await {
        info!("Message recieved from {}", &i.from);

        // Spin saftblandare for 4 secs
        #[cfg(feature = "rpi")]
        saftblandare.alert(Duration::from_secs(5)).await;

        // Send the message the the printer
        matrixprinter.print(i).await?;
    }

    Ok(())
}
