use crate::config::{CLIENT_PORT, SERVER_PORT, TIMEOUT};
use crate::error::throw;
use crate::key::Key;
use crate::message::Message;
use crate::peers::Peers;
use crate::utils::get_content_from_buffer;

use ansi_term::Colour::{Purple, Yellow};
use async_std::net::UdpSocket;
use async_std::{io, task};
use std::sync::Arc;
use std::time::Duration;

/// Starts the UDP client based on a tuple of peers and a crypto key.
pub async fn start_udp_client(peers: Arc<Peers>, key: Arc<Key>) -> Result<(), ()> {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        // Read a line from stdin.
        match stdin.read_line(&mut line).await {
            Ok(n) => {
                // End of stdin.
                if n == 0 {
                    return Ok(());
                }

                task::block_on(async {
                    send_udp_message(Arc::clone(&peers), &line, Arc::clone(&key)).await;
                });

                line.clear();
            }
            Err(_) => throw(301),
        }
    }
}

/// Starts the UDP server based on a tuple of peers and a crypto key.
pub async fn start_udp_server(peers: Arc<Peers>, key: Arc<Key>) {
    let mut buffer = vec![0u8; 1024];
    let key = &key;

    match UdpSocket::bind([peers.local.as_str(), ":", &SERVER_PORT.to_string()].join("")).await {
        Ok(socket) => loop {
            if let Ok(received) = socket.recv_from(&mut buffer).await {
                let (number_of_bytes, origin) = received;
                let message =
                    Message::deserialize(get_content_from_buffer(&buffer, number_of_bytes));

                match key.verify_message_signature(&message) {
                    Ok(_) => println!("⬅️ {}", Purple.paint(message.decrypt(key.clone()))),
                    Err(_) => throw(101),
                }

                match socket.send_to(&buffer[..number_of_bytes], &origin).await {
                    Ok(sent) => {
                        // TODO
                        // println!(
                        //     "Sent {} out of {} bytes to {}",
                        //     Purple.paint(sent.to_string().as_str()),
                        //     Purple.paint(n.to_string().as_str()),
                        //     Purple.paint(peer.to_string())
                        // );
                    }
                    Err(_) => throw(202),
                }
            }
        },
        Err(error) => eprintln!("{}", error),
    }
}

/// Send an UDP message to the first peer.
pub async fn send_udp_message(peers: Arc<Peers>, content: &str, key: Arc<Key>) {
    let cloned_key = key.clone();
    let message = Message::new(content.to_string(), cloned_key);

    match UdpSocket::bind([peers.local.as_str(), ":", &CLIENT_PORT.to_string()].join("")).await {
        Ok(socket) => {
            match socket
                .send_to(
                    message.serialize().as_bytes(),
                    [peers.remote.as_str(), ":", &SERVER_PORT.to_string()].join(""),
                )
                .await
            {
                Ok(_) => {
                    let mut buffer = vec![0u8; 1024];

                    match io::timeout(Duration::from_secs(TIMEOUT), async {
                        socket.recv_from(&mut buffer).await
                    })
                    .await
                    {
                        Ok(received) => {
                            let number_of_bytes = received.0;
                            let message = Message::deserialize(get_content_from_buffer(
                                &buffer,
                                number_of_bytes,
                            ));

                            println!("➡️ {}", Yellow.paint(message.decrypt(key.clone())));
                        }
                        Err(_) => throw(201),
                    }
                }
                Err(_) => throw(202),
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}
