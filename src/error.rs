use crate::terminal::{println};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("🔑 Crypto error: {0}!")]
    Crypto(String),
    #[error("💣 Network error: {0}!")]
    Network(String),
    #[error("⌨️ Stdin error: {0}!")]
    Stdin(String),
    #[error("❓Unknown error")]
    Unknown,
}

pub fn throw(code: u16) {
    let error = match code {
        // Crypto errors:
        101 => Error::Crypto(String::from("can't verify message signature")),
        102 => Error::Crypto(String::from("can't decode key")),
        // Network errors:
        201 => Error::Network(String::from("timeout, can't connect to peer")),
        202 => Error::Network(String::from("message not sent")),
        // Stdin errors:
        301 => Error::Stdin(String::from("can't read from command line")),
        _ => Error::Unknown,
    };

    println(true, error.to_string());
}
