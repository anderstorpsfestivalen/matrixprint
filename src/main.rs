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

async fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let opts: Opts = Opts::parse();

    // Open printer
    let mut mp = printer::Printer::open(&opts.printer_path).await.unwrap();

    let (mut c, mut rx) = conn::Connection::new(&opts.websocket)
        .await
        .unwrap();

    c.connect().await.unwrap();

    while let Some(i) = rx.recv().await {
        dbg!(&i);
        mp.print(i).await;
    }
}
