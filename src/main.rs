mod conn;
mod error;
mod light;
mod message;
mod printer;
use clap::{AppSettings, Clap};
use pretty_env_logger;
use std::env;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #&[clap(short, long, default_value = "/dev/usb/lp0")]
    printer_path: String,

    #[clap(
        short,
        long,
        default_value = "wss://mch.anderstorpsfestivalen.se/kernel/pipe"
    )]
    websocket: String,

    #[clap(short, long, default_value = 26)]
    relaypin: i32,
}

#[tokio::main]
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let opts: Opts = Opts::parse();

    // Open printer
    let mut mp = printer::Printer::open(&opts.printer_path).await.unwrap();

    let (mut c, mut rx) = conn::Connection::new(&opts.websocket)
        .await
        .unwrap();
    let mut matrixprinter = printer::Printer::open("/dev/usb/lp0").await?;

    // Initialize saftblandare
    let mut saftblandare = light::Light::init(23).await?;

    // Connect to backend
    let (mut c, mut rx) =
        conn::Connection::new("wss://mch.anderstorpsfestivalen.se/kernel/pipe").await?;
    c.connect().await?;

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
