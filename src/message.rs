use base64::decode;
use ring::digest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

impl Message {
    pub fn base64_decode(value: String) -> Result<[u8; digest::SHA384_OUTPUT_LEN], String> {
        match decode(&value) {
            Ok(value) => {
                let mut key_value = [0; digest::SHA384_OUTPUT_LEN];
                let to_array = &value[..key_value.len()];

                key_value.copy_from_slice(&value[..to_array.len()]);

                Ok(key_value)
            }
            Err(_) => Err(String::from("Can't base64 decode message!")),
        }
    }

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
