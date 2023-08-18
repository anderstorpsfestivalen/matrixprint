use crate::error::Error;
use crate::message::Message;
use crate::stats::Stats;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct Printer {
    output: tokio::fs::File,
    stats: Option<Stats>,
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
        let process = match Command::new("lpr")
            .arg("-P")
            .arg("lp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn lp: {}", why),
            Ok(process) => process,
        };

        let v: Vec<u8> = msg.into();

        match process.stdin.unwrap().write_all(&v) {
            Err(why) => panic!("couldn't write to lp stdin: {}", why),
            Ok(_) => println!("sent pangram to wc"),
        }

        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read lp stdout: {}", why),
            Ok(_) => print!("lp responded with:\n{}", s),
        }

        //Write newline
        // match self
        //     .output
        //     .write_all(&[ControlCodes::LineFeed.value()])
        //     .await
        // {
        //     Ok(_) => {}
        //     Err(e) => println!("{}", e),
        // }

        // let v: Vec<u8> = msg.into();

        // match self.output.write_all(&v).await {
        //     Ok(_) => {}
        //     Err(e) => println!("{}", e),
        // }

        // if let Some(s) = &self.stats {
        //     //s.print().await?;

        Ok(())
    }
}

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
