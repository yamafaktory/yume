mod async_utils;
mod config;
mod message;

use crate::async_utils::{read_from_stdin, start_udp_server};
use async_std::task;
use std::env::args;
use std::sync::Arc;

fn get_peers_ip() -> Result<(String, String), String> {
    if args().len() != 3 {
        return Err(String::from("Peer IP is missing"));
    }

    Ok((args().nth(1).unwrap(), args().nth(2).unwrap()))
}

fn main() {
    task::block_on(async {
        let peers_ip = get_peers_ip();

        if peers_ip.is_err() {
            eprintln!("{}", peers_ip.clone().unwrap_err());

            return;
        }

        let peers_ip = Arc::new(peers_ip.unwrap());
        let cloned_peers_ip = peers_ip.clone();

        task::spawn(async move {
            start_udp_server(cloned_peers_ip).await;
        });

        read_from_stdin(peers_ip.clone()).await.unwrap();
    });
}
