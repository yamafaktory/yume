use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

impl Message {
    pub fn new(content: String) -> Self {
        Message { content }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(message: String) -> Message {
        serde_json::from_str(message.as_str()).unwrap()
    }
}
