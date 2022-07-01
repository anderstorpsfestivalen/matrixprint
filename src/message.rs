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
