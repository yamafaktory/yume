use ansi_term::Colour::Red;
use base64::{decode, encode};
use ring::{digest, hmac};
use std::fmt;

use crate::message::Message;
use crate::utils::generate_random_array;

pub struct Key {
    pub value: [u8; digest::SHA512_OUTPUT_LEN],
    pub secret: hmac::Key,
}

impl Key {
    pub fn base64_decode(value: String) -> Result<[u8; digest::SHA512_OUTPUT_LEN], String> {
        match decode(&value) {
            Ok(value) => {
                let mut key_value = [0; digest::SHA512_OUTPUT_LEN];
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

    pub fn new(value: Option<[u8; digest::SHA512_OUTPUT_LEN]>) -> Self {
        let value = match value {
            Some(value) => value,
            None => generate_random_array(),
        };
        let key = Key {
            value,
            secret: hmac::Key::new(hmac::HMAC_SHA512, value.as_ref()),
        };

        // Print the newly generated key for reuse.
        println!(); // new line for readability
        println!("{}", key);
        println!();

        key
    }

    pub fn encode_message_signature(&self, message: Vec<u8>) -> String {
        encode(
            &hmac::sign(&self.secret, message.as_slice())
                .as_ref()
                .to_vec(),
        )
    }

    pub fn get_half_key_value(&self) -> [u8; 32] {
        // Resize key value from 64 to 32 to match GenericArray!
        let mut half_key_value = [0; 32];
        let to_array = &self.value[..half_key_value.len()];

        half_key_value.copy_from_slice(&self.value[..to_array.len()]);

        half_key_value
    }

    pub fn verify_message_signature(&self, message: &Message) -> Result<(), String> {
        let content = message.content.clone();
        let signature = self.encode_message_signature(content);

        if signature.as_str() == message.signature {
            Ok(())
        } else {
            Err(String::from("Invalid message signature!"))
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", Red.paint(self.base64_encode()))
    }
}
