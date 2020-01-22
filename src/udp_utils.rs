use async_std::net::UdpSocket;
use async_std::{io, task};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal,
};
use std::io::{stdout, Write};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{CLIENT_PORT, SERVER_PORT, TIMEOUT};
use crate::error::throw;
use crate::key::Key;
use crate::message::Message;
use crate::peers::Peers;
use crate::utils::get_content_from_buffer;
use crate::terminal::{println};

/// Starts the UDP client based on a tuple of peers and a crypto key.
pub async fn start_udp_client(peers: Arc<Peers>, key: Arc<Key>) {
    let mut characters = String::new();

    while let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
        let shared_characters = Arc::new(characters.clone());

        match code {
            KeyCode::Enter => {
                if characters.clone() == "/quit" {
                    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
                    terminal::disable_raw_mode().unwrap();
                    break;
                }

                task::block_on(async {
                    send_udp_message(
                        Arc::clone(&peers),
                        Arc::clone(&shared_characters),
                        Arc::clone(&key),
                    )
                    .await;
                });

                characters = String::new();
            }
            KeyCode::Char(character) => {
                characters.push(character);
                execute!(stdout(), Print(character)).unwrap();
            }
            KeyCode::Backspace => {
                characters.pop();
                execute!(
                    stdout(),
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    cursor::MoveToColumn(0),
                    Print(characters.clone()),
                )
                .unwrap();
            }

            _ => {}
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
                    Ok(_) => {
                        println(format!(
                            "{} {}",
                            peers.display_remote(),
                            message.decrypt(key.clone())
                        ));
                    }
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
pub async fn send_udp_message(peers: Arc<Peers>, content: Arc<String>, key: Arc<Key>) {
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

                    execute!(
                        stdout(),
                        cursor::Hide,
                        terminal::Clear(terminal::ClearType::CurrentLine),
                        cursor::MoveToColumn(0),
                        Print("⏳"),
                        cursor::MoveToColumn(0),
                    )
                    .unwrap();

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

                            execute!(
                                stdout(),
                                terminal::Clear(terminal::ClearType::CurrentLine),
                                cursor::MoveToColumn(0),
                                Print(message.decrypt(key.clone())),
                                Print("\n"),
                                cursor::MoveToColumn(0),
                                cursor::Show,
                            )
                            .unwrap();
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
