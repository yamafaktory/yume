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
use crate::help::render as render_help;
use crate::io::Line;
use crate::key::Key;
use crate::message::Message;
use crate::peers::Peers;
use crate::utils::get_content_from_buffer;

/// Starts the UDP client based on a tuple of peers and a crypto key.
pub async fn start(
    peers: Arc<Peers>,
    key: Arc<Key>,
    sender_receiver: Arc<(Sender<Option<Line>>, Receiver<Option<Line>>)>,
) {
    let mut characters = String::new();

    loop {
        let shared_characters = Arc::new(characters.clone());
        let sender_receiver = sender_receiver.clone();

        match event::read().unwrap() {
            Event::Mouse(_) => (),
            Event::Resize(_, _) => {
                execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
                terminal::disable_raw_mode().unwrap();
                throw(302);
                break;
            }
            Event::Key(KeyEvent { code, .. }) => {
                match code {
                    KeyCode::Enter => {
                        match characters.as_str() {
                            "/help" => render_help().await,
                            "/quit" => {
                                execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
                                terminal::disable_raw_mode().unwrap();
                                break;
                            }
                            _ => (),
                        };

                        let mut stdout = stdout();

                        for line in 0..Line::get_content_lines_length(characters) {
                            queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine),)
                                .unwrap();

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
                            queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine),)
                                .unwrap();

                            if line > 0 {
                                queue!(stdout, cursor::MoveUp(1),).unwrap();
                            }
                        }

                        queue!(
                            stdout,
                            cursor::MoveToColumn(0),
                            terminal::Clear(terminal::ClearType::CurrentLine),
                            Print(characters.clone())
                        )
                        .unwrap();

                        stdout.flush().unwrap();
                    }

                    _ => {}
                }
            }
        }
    }
}

/// Send an UDP message to the remote peer.
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
