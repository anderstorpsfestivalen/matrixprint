mod conn;
mod error;
mod light;
mod message;
mod printer;
use anyhow::Result;
use clap::Parser;
use log::info;
use pretty_env_logger;
use std::env;
use tokio::time::Duration;

#[derive(Parser, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    #[clap(short, long, default_value = "/dev/usb/lp0")]
    printer_path: String,

    #[clap(
        short,
        long,
        default_value = "wss://mch.anderstorpsfestivalen.se/kernel/pipe"
    )]
    websocket: String,

    #[clap(short, long, default_value = "26")]
    relaypin: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let opts = Args::parse();

    // Open printer
    let mut matrixprinter = printer::Printer::open(&opts.printer_path).await?;

    // Initialize saftblandare
    let mut saftblandare = light::Light::init(opts.relaypin).await?;

    // Connect to backend
    let (mut c, mut rx) = conn::Connection::new(&opts.websocket).await?;
    c.connect().await?;

    // Forever ?
    while let Some(i) = rx.recv().await {
        info!("Message recieved from {}", &i.from);
        // Spin saftblandare for 4 secs
        saftblandare.alert(Duration::from_secs(5)).await;

        // Send the message the the printer
        matrixprinter.print(i).await?;
    }

    Ok(())
}
