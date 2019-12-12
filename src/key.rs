use base64::{decode, encode};
use ring::{digest, hmac, rand};
use std::fmt;

use crate::message::Message;

pub struct Key {
    pub value: [u8; digest::SHA384_OUTPUT_LEN],
    pub secret: hmac::Key,
}

impl Key {
    pub fn base64_decode(value: String) -> Result<[u8; digest::SHA384_OUTPUT_LEN], String> {
        match decode(&value) {
            Ok(value) => {
                let mut key_value = [0; digest::SHA384_OUTPUT_LEN];
                let to_array = &value[..key_value.len()];

                key_value.copy_from_slice(&value[..to_array.len()]);

                Ok(key_value)
            }
            Err(_) => Err(String::from("Can't decode key!")),
        }
    }

    pub fn base64_encode(&self) -> String {
        encode(&self.value.to_vec())
    }

    pub fn new(value: Option<[u8; digest::SHA384_OUTPUT_LEN]>) -> Self {
        let value = match value {
            Some(value) => value,
            None => Key::generate_hmac_value(),
        };

        Key {
            value,
            secret: hmac::Key::new(hmac::HMAC_SHA256, value.as_ref()),
        }
    }

    pub fn encode_message_signature(&self, message: String) -> String {
        encode(
            &hmac::sign(&self.secret, message.as_bytes())
                .as_ref()
                .to_vec(),
        )
    }

    pub fn generate_hmac_value() -> [u8; digest::SHA384_OUTPUT_LEN] {
        let random = rand::SystemRandom::new();
        let key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&random).unwrap().expose();

        key_value
    }

    pub fn verify_message_signature(&self, message: &Message) -> Result<(), String> {
        let content = message.content.clone();
        let signature = self.encode_message_signature(content);

        if signature.as_str() == message.signature {
            Ok(())
        } else {
            Err(String::from("Can't base64 decode message signature!"))
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.base64_encode())
    }
}
