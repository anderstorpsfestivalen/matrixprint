mod conn;
mod error;
mod message;

#[tokio::main]

async fn main() {
    let (mut c, mut rx) = conn::Connection::new("wss://mch.anderstorpsfestivalen.se/kernel/pipe")
        .await
        .unwrap();

    c.connect().await.unwrap();
}
