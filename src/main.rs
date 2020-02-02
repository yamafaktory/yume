mod config;
mod error;
mod key;
mod message;
mod peers;
mod terminal;
mod udp_utils;
mod utils;

use crate::error::throw;
use crate::key::Key;
use crate::peers::Peers;
use crate::terminal::{enter_secondary_screen, prompt};
use crate::udp_utils::{start_udp_client, start_udp_server};

use async_std::sync::{channel, Receiver, Sender};
use async_std::task;
use std::env::args;
use std::sync::Arc;

fn get_peers() -> Result<Peers, String> {
    if args().len() != 3 {
        return Err(String::from("Peer IP is missing"));
    }

    Ok(Peers::new(args().nth(1).unwrap(), args().nth(2).unwrap()))
}

fn main() {
    task::block_on(async {
        let current_peers = get_peers();

        if let Err(error) = current_peers {
            eprintln!("{}", error);

            return;
        }

        let peers = Arc::new(current_peers.unwrap());
        let cloned_peers = peers.clone();

        enter_secondary_screen();

        let secret_key = prompt(Some(String::from(
            "🔑 Enter secret key or click enter to generate a new one:",
        )));

        let key = match secret_key {
            Ok(secret_key) => Key::new(if secret_key.is_empty() {
                None
            } else {
                match Key::base64_decode(secret_key) {
                    Ok(secret_key) => Some(secret_key),
                    Err(code) => {
                        throw(code);

                        return;
                    }
                }
            }),
            Err(error) => {
                eprintln!("{}", error);

                return;
            }
        };

        let key = Arc::new(key);
        let cloned_key = key.clone();

        let sender_receiver: (Sender<Option<String>>, Receiver<Option<String>>) = channel(1);
        let cloned_sender_receiver = sender_receiver.clone();

        task::spawn(async move {
            start_udp_server(cloned_peers, cloned_key, sender_receiver).await;
        });

        start_udp_client(peers.clone(), key, cloned_sender_receiver).await;
    });
}
