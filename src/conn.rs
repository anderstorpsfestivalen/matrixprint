use crate::error::Error;
use crate::message::Message as msg;
use log::error;
use url::Url;

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct Connection {
    host: Url,

    pipe: tokio::sync::mpsc::Sender<msg>,
}

impl Connection {
    pub async fn new(dst: &str) -> Result<(Connection, tokio::sync::mpsc::Receiver<msg>), Error> {
        let url = Url::parse(dst)?;

        let (tx, rx) = mpsc::channel(100);

        Ok((
            Connection {
                host: url,
                pipe: tx,
            },
            rx,
        ))
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let (ws_stream, rs) = connect_async(&self.host).await?;

        dbg!(rs);

        let p = self.pipe.clone();
        tokio::spawn(Connection::process(ws_stream, p));

        Ok(())
    }

    async fn process(
        ws_stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        input: tokio::sync::mpsc::Sender<msg>,
    ) {
        ws_stream
            .for_each(|message| async {
                let body = match message {
                    Ok(v) => v,
                    Err(e) => {
                        error!("could not unpack message body: {}", e);
                        return;
                    }
                };

                let m: Result<msg, serde_json::Error> = serde_json::from_slice(&body.into_data());

                match m {
                    Ok(message) => match input.send(message).await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("could not send message to channel: {}", e);
                        }
                    },
                    Err(e) => {
                        error!("could not parse message: {}", e);
                        return;
                    }
                }
            })
            .await;
    }
}
