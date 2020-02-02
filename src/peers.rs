use crossterm::{execute, style};
use std::io::{stdout, Write};

#[derive(Clone)]
pub struct Peers {
    pub local: String,
    pub remote: String,
}

impl Peers {
    pub fn new(local: String, remote: String) -> Self {
        Peers { local, remote }
    }

    pub fn display_remote(&self) {
        execute!(
            stdout(),
            style::SetForegroundColor(style::Color::DarkMagenta),
            style::Print(format!("{} ", self.local.clone())),
            style::SetForegroundColor(style::Color::White)
        )
        .unwrap();
    }
}
