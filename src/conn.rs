use crate::error::Error;
use crate::message::Message as msg;
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

        ws_stream
            .for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
            .await;

        Ok(())
    }
}
