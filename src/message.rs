use crate::cc;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Filtered")]
    pub filtered: String,
}

impl Into<Vec<u8>> for Message {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::new();
        // From
        v.extend_from_slice(&[cc::ControlCodes::Wide.into()]);
        v.extend_from_slice(b"FROM: ");
        v.extend_from_slice(&[cc::ControlCodes::ReleaseWide.into()]);
        v.extend_from_slice(&self.from.as_bytes());
        v.extend_from_slice(b"\n");

        v.extend_from_slice(&[cc::ControlCodes::Wide.into()]);
        v.extend_from_slice(b"SUBJECT: ");
        v.extend_from_slice(&[cc::ControlCodes::ReleaseWide.into()]);
        v.extend_from_slice(&self.subject.as_bytes());
        v.extend_from_slice(b"\n");

        v.extend_from_slice(b"MESSAGE: ");
        v.extend_from_slice(b"\n");
        v.extend_from_slice(&[cc::ControlCodes::Compressed.into()]);
        v.extend_from_slice(&self.filtered.as_bytes());
        v.extend_from_slice(&[cc::ControlCodes::ReleaseCompressed.into()]);
        v.extend_from_slice(b"\n");

        v.extend_from_slice(&[cc::ControlCodes::FormFeed.into()]);
        v
    }
}
