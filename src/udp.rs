use async_std::io;
use async_std::net::UdpSocket;
use async_std::sync::{Receiver, Sender};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::Print,
    terminal,
};
use std::io::{stdout, Write};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{BUFFER_SIZE, CLIENT_PORT, SERVER_PORT, TIMEOUT};
use crate::error::throw;
use crate::io::Line;
use crate::key::Key;
use crate::message::Message;
use crate::peers::Peers;
use crate::terminal::println;
use crate::utils::get_content_from_buffer;

/// Starts the UDP client based on a tuple of peers and a crypto key.
pub async fn start_client(
    peers: Arc<Peers>,
    key: Arc<Key>,
    sender_receiver: Arc<(Sender<Option<Line>>, Receiver<Option<Line>>)>,
) {
    let mut characters = String::new();

    while let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
        let shared_characters = Arc::new(characters.clone());
        let sender_receiver = sender_receiver.clone();

        match code {
            KeyCode::Enter => {
                match characters.as_str() {
                    "/help" => println(String::from("TODO"), false),
                    "/quit" => {
                        execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        break;
                    }
                    _ => (),
                };

                let mut stdout = stdout();

                for line in 0..Line::get_content_lines_length(characters) {
                    queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine),).unwrap();

                    if line > 0 {
                        queue!(stdout, cursor::MoveUp(1)).unwrap();
                    }
                }

                // Push a noop value in the channel.
                if !sender_receiver.1.is_empty() {
                    sender_receiver.1.recv().await;
                }
                sender_receiver.0.send(None).await;

                // Send message.
                send_message(
                    Arc::clone(&peers),
                    Arc::clone(&shared_characters),
                    Arc::clone(&key),
                )
                .await;

                // Reset afterwards.
                characters = String::new();
            }
            KeyCode::Char(character) => {
                characters.push(character);

                if !sender_receiver.1.is_empty() {
                    sender_receiver.1.recv().await;
                }
                sender_receiver
                    .0
                    .send(Some(Line::new(characters.clone())))
                    .await;

                execute!(stdout(), Print(character)).unwrap();
            }
            KeyCode::Backspace => {
                characters.pop();

                if !sender_receiver.1.is_empty() {
                    sender_receiver.1.recv().await;
                }
                sender_receiver
                    .0
                    .send(Some(Line::new(characters.clone())))
                    .await;

                let mut stdout = stdout();

                for line in 0..Line::get_content_lines_length(characters.clone()) {
                    queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine),).unwrap();

                    if line > 0 {
                        queue!(stdout, cursor::MoveUp(1),).unwrap();
                    }
                }
                queue!(stdout, cursor::MoveToColumn(0)).unwrap();
                queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine),).unwrap();
                queue!(stdout, Print(characters.clone()),).unwrap();
                stdout.flush().unwrap();
            }

            _ => {}
        }
    }
}

/// Starts the UDP server based on a tuple of peers and a crypto key.
pub async fn start_server(
    peers: Arc<Peers>,
    key: Arc<Key>,
    sender_receiver: Arc<(Sender<Option<Line>>, Receiver<Option<Line>>)>,
) {
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

                                            // We want to replay the line in the channel afterwards, store it.
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
                        Err(_) => throw(101),
                    }
                }
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}

/// Send an UDP message to the first peer.
pub async fn send_message(peers: Arc<Peers>, content: Arc<String>, key: Arc<Key>) {
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
                    let mut buffer = vec![0u8; BUFFER_SIZE];

                    execute!(
                        stdout(),
                        cursor::Hide,
                        terminal::Clear(terminal::ClearType::CurrentLine),
                        cursor::MoveToColumn(0),
                        Print("Sending message..."),
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

                            match Message::deserialize(get_content_from_buffer(
                                &buffer,
                                number_of_bytes,
                            )) {
                                Ok(message) => {
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
                                Err(_) => throw(401),
                            }
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
