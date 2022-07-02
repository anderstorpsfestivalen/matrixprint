use crate::error::Error;
use crate::message::Message;
use futures_util::StreamExt;
use log::{error, info, warn};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::{connect_async, tungstenite::Error as TE};
use url::Url;

pub struct Connection {
    pipe: Sender<Message>,
}

impl Connection {
    pub async fn new(dst: &str) -> Result<(Connection, Receiver<Message>), Error> {
        let url = Url::parse(dst)?;

        // Data pipe
        let (tx, rx) = mpsc::channel(100);

        let conn = Connection { pipe: tx };

        let p = conn.pipe.clone();
        tokio::spawn(async { Connection::connect(url, p).await });

        Ok((conn, rx))
    }

    pub async fn connect(url: Url, pipe: Sender<Message>) -> Result<(), Error> {
        // Reconnect pipe
        let (ctx, mut crx) = mpsc::channel(100);

        loop {
            let (ws_stream, mv) = match connect_async(&url).await {
                Ok(v) => v,
                Err(e) => {
                    error!("could not connect, reconnecting {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            info!("connected to websocket {}, {}", &url, mv.status());

            let p = pipe.clone();
            let c = ctx.clone();
            tokio::spawn(Connection::process(ws_stream, p, c));

            crx.recv().await;

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    async fn process(
        ws_stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        input: Sender<Message>,
        cancel: Sender<()>,
    ) {
        ws_stream
            .for_each(|message| async {
                let body = match message {
                    Ok(v) => v,
                    Err(e) => {
                        match e {
                            TE::ConnectionClosed | TE::AlreadyClosed | TE::Protocol(_) => {
                                warn!("lost websocket connection: {}", e);
                            }
                            _ => {
                                error!("could not unpack message body: {}", e);
                            }
                        }
                        return;
                    }
                };

                let m: Result<Message, serde_json::Error> =
                    serde_json::from_slice(&body.into_data());

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

        match cancel.send(()).await {
            Err(e) => {
                panic!(
                    "Cannot cancel the websocket process thread (??), panic. {}",
                    e
                );
            }
            _ => {}
        }
    }
}
