use crate::config::{CLIENT_PORT, ERROR_MESSAGES, SERVER_PORT};
use crate::message::Message;

use ansi_term::Colour::{Purple, Yellow};
use async_std::net::UdpSocket;
use async_std::{io, task};
use std::sync::Arc;
use std::time::Duration;

/// Some doc.
pub async fn start_udp_server(peers_ip: Arc<(String, String)>) {
    let mut buffer = vec![0u8; 1024];

    match UdpSocket::bind([peers_ip.0.as_str(), ":", &SERVER_PORT.to_string()].join("")).await {
        Ok(socket) => loop {
            if let Ok(received) = socket.recv_from(&mut buffer).await {
                let (n, peer) = received;
                let p = Message::deserialize(String::from_utf8_lossy(&buffer[..n]).to_string());

                println!("⬅️ {}", Purple.paint(p.content));

                match socket.send_to(&buffer[..n], &peer).await {
                    Ok(sent) => {
                        // println!(
                        //     "Sent {} out of {} bytes to {}",
                        //     Purple.paint(sent.to_string().as_str()),
                        //     Purple.paint(n.to_string().as_str()),
                        //     Purple.paint(peer.to_string())
                        // );
                    }
                    Err(_) => eprint!("Error!"),
                }
            }
        },
        Err(error) => eprintln!("{}", error),
    }
}

pub async fn send_udp_message(peers_ip: Arc<(String, String)>, content: &str) {
    let m = Message::new(content.to_string());

    match UdpSocket::bind([peers_ip.0.as_str(), ":", &CLIENT_PORT.to_string()].join("")).await {
        Ok(socket) => {
            match socket
                .send_to(
                    m.serialize().as_bytes(),
                    [peers_ip.1.as_str(), ":", &SERVER_PORT.to_string()].join(""),
                )
                .await
            {
                Ok(_) => {
                    let mut buffer = vec![0u8; 1024];

                    match io::timeout(Duration::from_secs(5), async {
                        socket.recv_from(&mut buffer).await
                    })
                    .await
                    {
                        Ok(received) => {
                            let (n, _) = received;

                            println!("➡️ {}", Yellow.paint(String::from_utf8_lossy(&buffer[..n])));
                        }
                        Err(_) => {
                            eprintln!("{}", ERROR_MESSAGES[0]);
                        }
                    }
                }
                Err(_) => println!("Error message not sent"),
            }
        }
        Err(error) => println!("{}", error),
    }
}

pub async fn read_from_stdin(peers_ip: Arc<(String, String)>) -> Result<(), ()> {
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
                    send_udp_message(Arc::clone(&peers_ip), &line).await;
                });

                line.clear();
            }
            Err(_) => eprintln!("gnii"),
        }
    }
}
