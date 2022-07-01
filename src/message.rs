use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub date: String,

    pub text: String,
    pub filtered: String,
}
