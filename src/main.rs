mod client;
mod config;
mod error;
mod io;
mod key;
mod message;
mod peers;
mod server;
mod terminal;
mod utils;

use crate::client::start as start_client;
use crate::config::{DESCRIPTION, VERSION};
use crate::error::throw;
use crate::io::Line;
use crate::key::Key;
use crate::peers::Peers;
use crate::server::start as start_server;
use crate::terminal::{enter_secondary_screen, println, prompt};

use async_std::sync::{channel, Receiver, Sender};
use async_std::task;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::env::args;
use std::io::{stdout, Write};
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

        println(String::from(DESCRIPTION), true);
        println(format!("Version {}\n", VERSION), true);

        let secret_key = prompt(Some(String::from(
            "Enter secret key or press enter to generate a new one:",
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

        println(String::from("\nYou can start typing!\n"), true);

        let key = Arc::new(key);
        let cloned_key = key.clone();

        let sender_receiver: Arc<(Sender<Option<Line>>, Receiver<Option<Line>>)> =
            Arc::new(channel(1));
        let cloned_sender_receiver = sender_receiver.clone();

        task::spawn(async move {
            start_server(cloned_peers, cloned_key, sender_receiver).await;
        });

        start_client(peers.clone(), key, cloned_sender_receiver).await;
    });
}
