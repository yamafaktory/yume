use aead::{generic_array::GenericArray, Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;
use serde::{Deserialize, Serialize};
use std::str;
use std::sync::Arc;

use crate::key::Key;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: String,
}

impl Message {
    pub fn new(content: String, key: Arc<Key>) -> Self {
        // Resize key value from 64 to 32 to match GenericArray!
        let mut half_key_value = [0; 32];
        let to_array = &key.value[..half_key_value.len()];
        half_key_value.copy_from_slice(&key.value[..to_array.len()]);

        let key_value = GenericArray::clone_from_slice(&half_key_value);
        let aead = ChaCha20Poly1305::new(key_value);

        // TODO: nonce should be unique per message.
        let nonce = GenericArray::from_slice(b"unique nonce");

        let ciphertext = aead
            .encrypt(nonce, content.as_ref())
            .expect("encryption failure!");
        let t = ciphertext.clone();
        Message {
            content: ciphertext,
            nonce: nonce.to_vec(),
            signature: key.encode_message_signature(t),
        }
    }

    pub fn decrypt(&self, key: Arc<Key>) -> String {
        // Resize key value from 64 to 32 to match GenericArray!
        let mut half_key_value = [0; 32];
        let to_array = &key.value[..half_key_value.len()];
        half_key_value.copy_from_slice(&key.value[..to_array.len()]);

        let key_value = GenericArray::clone_from_slice(&half_key_value);
        let aead = ChaCha20Poly1305::new(key_value);

        // TODO: nonce should be unique per message.
        let nonce = GenericArray::from_slice(b"unique nonce");

        let plaintext = aead
            .decrypt(nonce, self.content.as_ref())
            .expect("decryption failure!");
        str::from_utf8(&plaintext).unwrap().to_string()
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(message: String) -> Message {
        serde_json::from_str(message.as_str()).unwrap()
    }
}
