use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum PayloadData {
    Message(MessagePayload),
    Spacer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    date: DateTime<Utc>,
    #[serde(flatten)]
    payload_data: PayloadData,
}

impl Payload {
    pub fn new(payload_data: PayloadData) -> Self {
        Self {
            date: Utc::now(),
            payload_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub message: String,
}

impl MessagePayload {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacerPayload {}

impl SpacerPayload {
    pub fn new() -> Self {
        Self {}
    }
}
