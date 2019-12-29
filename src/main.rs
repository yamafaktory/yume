mod config;
mod error;
mod key;
mod message;
mod peers;
mod stdin_utils;
mod udp_utils;
mod utils;

use crate::key::Key;
use crate::peers::Peers;
use crate::stdin_utils::prompt;
use crate::udp_utils::{start_udp_client, start_udp_server};

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

        let secret_key = prompt(Some(String::from(
            "🔑 Enter secret key or click enter to generate a new one:",
        )))
        .await;

        let key = match secret_key {
            Ok(secret_key) => Key::new(if secret_key.is_empty() {
                None
            } else {
                match Key::base64_decode(secret_key) {
                    Ok(secret_key) => Some(secret_key),
                    Err(error) => {
                        eprintln!("{}", error);

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

        task::spawn(async move {
            start_udp_server(cloned_peers, cloned_key).await;
        });

        start_udp_client(peers.clone(), key).await.unwrap();
    });
}
