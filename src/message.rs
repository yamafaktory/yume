use aead::{generic_array::GenericArray, Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;
use serde::{Deserialize, Serialize};
use std::str;
use std::sync::Arc;

use crate::config::NONCE_LENGTH;
use crate::key::Key;
use crate::utils::generate_random_array;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: String,
}

impl Message {
    pub fn new(content: String, key: Arc<Key>) -> Self {
        let key_value =
            GenericArray::clone_from_slice(&key.get_half_key_value());
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
        let key_value =
            GenericArray::clone_from_slice(&key.get_half_key_value());
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

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(message: String) -> Result<Message, ()> {
        match serde_json::from_str(message.as_str()) {
            Ok(message) => Ok(message),
            Err(_) => Err(()),
        }
    }
}
