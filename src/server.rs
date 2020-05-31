use async_std::net::UdpSocket;
use crossterm::{cursor, queue, style::Print, terminal};
use std::{
    io::{stdout, Write},
    sync::Arc,
};

use crate::{
    config::{BUFFER_SIZE, SERVER_PORT},
    error::throw,
    key::Key,
    message::Message,
    peers::Peers,
    terminal::println,
    types::SenderReceiver,
    utils::get_content_from_buffer,
};

/// Starts the UDP server based on a tuple of peers and a crypto key.
pub async fn start(peers: Arc<Peers>, key: Arc<Key>, sender_receiver: SenderReceiver) {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let key = &key;

    match UdpSocket::bind([peers.local.as_str(), ":", &SERVER_PORT.to_string()].join("")).await {
        Ok(socket) => {
            loop {
                if let Ok(received) = socket.recv_from(&mut buffer).await {
                    let (number_of_bytes, origin) = received;

                    match Message::deserialize(get_content_from_buffer(&buffer, number_of_bytes)) {
                        Ok(message) => {
                            // Display the message or throw an error.
                            match key.verify_message_signature(&message) {
                                Ok(_) => {
                                    let mut replay_line = None;
                                    let mut stdout = stdout();

                                    if !sender_receiver.1.is_empty() {
                                        if let Some(line) = sender_receiver.1.recv().await.unwrap()
                                        {
                                            let raw_line = line.clone();
                                            let arc_line = Arc::new(line);
                                            let arc_cloned_line = arc_line.clone();

                                            // We want to replay the line in the channel afterwards,
                                            // store it.
                                            replay_line = Some(arc_line);

                                            // Push it back in case we need to replay it again!
                                            sender_receiver.0.send(Some(raw_line)).await;

                                            for position in 0..arc_cloned_line.length {
                                                if position > 0 {
                                                    queue!(stdout, cursor::MoveUp(1),).unwrap();
                                                }

                                                queue!(
                                                    stdout,
                                                    terminal::Clear(
                                                        terminal::ClearType::CurrentLine
                                                    ),
                                                    cursor::MoveToColumn(0)
                                                )
                                                .unwrap();
                                            }
                                        }
                                    }

                                    // Display prepended peer I.P. and decrypted message.
                                    peers.display_remote();
                                    println(message.decrypt(key.clone()), false);

                                    if let Some(line) = replay_line {
                                        queue!(stdout, Print(&line.content)).unwrap();
                                    }

                                    stdout.flush().unwrap();
                                }
                                Err(_) => throw(401),
                            }

                            match socket.send_to(&buffer[..number_of_bytes], &origin).await {
                                Ok(_) => (),
                                Err(_) => throw(202),
                            }
                        }
                        Err(_) => throw(101),
                    }
                }
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}
