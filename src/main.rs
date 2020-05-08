mod client;
mod config;
mod error;
mod help;
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
use std::sync::Arc;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(
        help = "local and remote IPv6 peer addresses",
        min_values = 2,
        required = true
    )]
    peers: Vec<String>,
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let peers_from_args = Opt::from_args().peers;
    let current_peers = Peers::new(peers_from_args[0].clone(), peers_from_args[1].clone());

    let peers = Arc::new(current_peers);
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

                    return Ok(());
                }
            }
        }),
        Err(error) => {
            eprintln!("{}", error);

            return Ok(());
        }
    };

    println(String::from("\nYou can start typing!\n"), true);

    let key = Arc::new(key);
    let cloned_key = key.clone();

    let sender_receiver: Arc<(Sender<Option<Line>>, Receiver<Option<Line>>)> = Arc::new(channel(1));
    let cloned_sender_receiver = sender_receiver.clone();

    task::spawn(async move {
        start_server(cloned_peers, cloned_key, sender_receiver).await;
    });

    start_client(peers.clone(), key, cloned_sender_receiver).await;

    Ok(())
}
