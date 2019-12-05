use ring::{digest, hmac, rand};

pub struct Key {
    pub value: [u8; digest::SHA384_OUTPUT_LEN],
    pub secret: hmac::Key,
}

impl Key {
    pub fn new(value: Option<[u8; digest::SHA384_OUTPUT_LEN]>) -> Self {
        let value = match value {
            Some(value) => value,
            None => Key::generate_value(),
        };

        Key {
            value,
            secret: hmac::Key::new(hmac::HMAC_SHA256, value.as_ref()),
        }
    }

    pub fn encrypt_message(&self, message: String) -> String {
        let tag = hmac::sign(&self.secret, message.as_bytes());

        dbg!(tag);

        "encrypted".to_string()
    }

    pub fn generate_value() -> [u8; digest::SHA384_OUTPUT_LEN] {
        let random = rand::SystemRandom::new();
        let key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&random).unwrap().expose();
        // let secret_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
        println!("{:?}", key_value.to_vec());
        key_value
    }

    pub fn verify_message(&self, message: String) -> Result<(), String> {
        let tag = hmac::sign(&self.secret, message.as_bytes());
        let verify_key = hmac::Key::new(hmac::HMAC_SHA256, self.value.as_ref());

        match hmac::verify(&verify_key, message.as_bytes(), tag.as_ref()) {
            Ok(_) => Ok(()),
            Err(_) => Err("encrypted".to_string()),
        }
    }
}
