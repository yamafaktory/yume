use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::key::Key;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub signature: String,
}

impl Message {
    pub fn new(content: String, key: Arc<Key>) -> Self {
        let cloned_content = content.clone();

        Message {
            content,
            signature: key.encode_message_signature(cloned_content),
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(message: String) -> Message {
        serde_json::from_str(message.as_str()).unwrap()
    }
}
