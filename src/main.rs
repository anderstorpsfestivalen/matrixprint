mod conn;
mod error;
mod message;
mod printer;
use pretty_env_logger;
use std::env;

#[tokio::main]

async fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Open printer
    let mut mp = printer::Printer::open("/dev/usb/lp0").await.unwrap();

    let (mut c, mut rx) = conn::Connection::new("wss://mch.anderstorpsfestivalen.se/kernel/pipe")
        .await
        .unwrap();

    c.connect().await.unwrap();

    while let Some(i) = rx.recv().await {
        dbg!(&i);
        mp.print(i).await;
    }
}
