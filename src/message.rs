use ring::{digest, hmac, rand};
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
    pub fn encrypt(message: String) -> String {
        let rng = rand::SystemRandom::new();
        let key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&rng).unwrap().expose();
        dbg!(key_value.iter());
        let s_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
        let tag = hmac::sign(&s_key, message.as_bytes());
        dbg!(tag);
        "sdf".to_string()
    }
}
