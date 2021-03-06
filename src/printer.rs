use crate::error::Error;
use crate::stats::Stats;
use crate::message::Message;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct Printer {
    output: tokio::fs::File,
    stats: Option<Stats>
}

impl Printer {
    pub async fn open(path: &str, stats: Option<Stats>) -> Result<Printer, Error> {
        let output = OpenOptions::new()
            .read(false)
            .write(true)
            .create(false)
            .append(true)
            .open(path)
            .await?;

        Ok(Printer { output, stats })
    }

    pub async fn print(&mut self, msg: Message) -> Result<(), Error> {
        //Write newline
        self.output
            .write_all(&[ControlCodes::LineFeed.value()])
            .await?;

        self.output.write_all(b"hello, world!").await?;

        if let Some(s) = &self.stats {
            s.print().await?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub enum ControlCodes {
    Backspace,
    Cancel,
    CarriageReturn,
    Delete,
    FormFeed,
    HorizontalTab,
    LineFeed,
    Null,
    Compressed,
    ReleaseCompressed,
    Wide,
    ReleaseWide,
    VerticalTab,
}

impl ControlCodes {
    fn value(self) -> u8 {
        return match self {
            ControlCodes::Backspace => 0x08,
            ControlCodes::Cancel => 0x18,
            ControlCodes::CarriageReturn => 0x0D,
            ControlCodes::Delete => 0x7F,
            ControlCodes::FormFeed => 0x0C,
            ControlCodes::HorizontalTab => 0x09,
            ControlCodes::LineFeed => 0x0A,
            ControlCodes::Null => 0x00,
            ControlCodes::Compressed => 0x0F,
            ControlCodes::ReleaseCompressed => 0x12,
            ControlCodes::Wide => 0x0E,
            ControlCodes::ReleaseWide => 0x14,
            ControlCodes::VerticalTab => 0x0B,
        };
    }
}
