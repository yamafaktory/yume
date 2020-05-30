use crate::terminal::println;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Crypto error: {0}!")]
    Crypto(String),
    #[error("Message error: {0}!")]
    Message(String),
    #[error("Network error: {0}!")]
    Network(String),
    #[error("Stdin error: {0}!")]
    Stdin(String),
    #[error("Unknown error!")]
    Unknown,
}

fn _throw(code: u16) -> Error {
    match code {
        // Crypto errors:
        101 => Error::Crypto(String::from("can't verify message signature")),
        102 => Error::Crypto(String::from("can't decode key")),
        // Network errors:
        201 => Error::Network(String::from("timeout, can't connect to peer")),
        202 => Error::Network(String::from("message not sent")),
        // Stdin errors:
        301 => Error::Stdin(String::from("can't read from command line")),
        302 => Error::Stdin(String::from("resizing is unsupported")),
        // Message errors:
        401 => Error::Message(String::from("can't deserialize message")),
        _ => Error::Unknown,
    }
}

pub fn throw(code: u16) { println(_throw(code).to_string(), true); }

#[cfg(test)]
mod utils {
    use super::*;

    #[test]
    fn test_error() {
        assert_eq!(
            _throw(101).to_string(),
            String::from("Crypto error: can't verify message signature!")
        );

        assert_eq!(_throw(999).to_string(), String::from("Unknown error!"));
    }
}
