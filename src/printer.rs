use crate::error::Error;
use crate::message::Message;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct Printer {
    output: tokio::fs::File,
}

impl Printer {
    pub async fn open(path: &str) -> Result<Printer, Error> {
        let output = OpenOptions::new()
            .read(false)
            .write(true)
            .create(false)
            .append(true)
            .open(path)
            .await?;

        Ok(Printer { output })
    }

    pub async fn print(&mut self, msg: Message) -> Result<(), Error> {
        self.output.write_all(b"hello, world!").await?;

        Ok(())
    }
}
