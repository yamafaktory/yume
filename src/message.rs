use aead::{generic_array::GenericArray, Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;
use serde::{Deserialize, Serialize};
use std::{str, sync::Arc};

use crate::{config::NONCE_LENGTH, key::Key, utils::generate_random_array};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    pub content: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: String,
}

impl Message {
    pub fn new(content: String, key: Arc<Key>) -> Self {
        let key_value = GenericArray::clone_from_slice(&key.get_half_key_value());
        let aead = ChaCha20Poly1305::new(key_value);
        let nonce_array = generate_random_array();
        let nonce = GenericArray::from_slice(&nonce_array[0..NONCE_LENGTH]);
        let ciphertext = aead
            .encrypt(nonce, content.as_ref())
            .expect("encryption failure!");
        let cloned_ciphertext = ciphertext.clone();

        Message {
            content: ciphertext,
            nonce: nonce.to_vec(),
            signature: key.encode_message_signature(cloned_ciphertext),
        }
    }

    pub fn decrypt(&self, key: Arc<Key>) -> String {
        let key_value = GenericArray::clone_from_slice(&key.get_half_key_value());
        let aead = ChaCha20Poly1305::new(key_value);

        let mut nonce_from_vec = [0; NONCE_LENGTH];
        let to_array = &self.nonce[..nonce_from_vec.len()];

        nonce_from_vec.copy_from_slice(&self.nonce[..to_array.len()]);

        let plaintext = aead
            .decrypt(
                GenericArray::from_slice(&nonce_from_vec),
                self.content.as_ref(),
            )
            .expect("decryption failure!");

        str::from_utf8(&plaintext).unwrap().to_string()
    }

    pub fn serialize(&self) -> String { serde_json::to_string(self).unwrap() }

    pub fn deserialize(message: String) -> Result<Message, ()> {
        match serde_json::from_str(message.as_str()) {
            Ok(message) => Ok(message),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod utils {
    use super::*;

    #[test]
    fn test_message() {
        let key = Key::new(None);
        let cloned_key = key.clone();
        let cloned_cloned_key = key.clone();
        let message_a = Message::new(String::from("foo"), Arc::new(key));
        let message_b = Message::new(String::from("foo"), Arc::new(cloned_key));

        // Both messages' contents should be different since they are based on different
        // nonce!
        assert_ne!(message_a.content, message_b.content);
        // They should have the same length.
        assert_eq!(message_a.content.len(), message_b.content.len());
        // The nonces should be different too!
        assert_ne!(message_a.nonce, message_b.nonce);
        // They should have the same length.
        assert_eq!(message_a.nonce.len(), message_b.nonce.len());
        // The signatures should also be different!
        assert_ne!(message_a.signature, message_b.signature);
        // They should have the same length.
        assert_eq!(message_a.signature.len(), message_b.signature.len());

        // Decrypting a message should returns its content.
        assert_eq!(
            message_a.decrypt(Arc::new(cloned_cloned_key)),
            String::from("foo")
        );

        // Serializing and deserializaing a message should not alter it.
        assert_eq!(
            Message::deserialize(message_a.serialize()).unwrap(),
            message_a
        );
    }
}
