use crossterm::{cursor, execute, queue, style, style::Print, terminal};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    io::{stdout, Write},
};

lazy_static! {
    static ref COMMANDS: HashMap<&'static str, &'static str> =
        vec![("help", "display help"), ("quit", "quit application"),]
            .into_iter()
            .collect();
}

pub async fn render() {
    let mut stdout = stdout();

    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::CurrentLine),
        Print("\n")
    )
    .unwrap();

    for (key, value) in COMMANDS.iter() {
        execute!(
            stdout,
            cursor::MoveToColumn(0),
            style::SetForegroundColor(style::Color::DarkYellow),
            style::Print(format!("{} ", key)),
            style::SetForegroundColor(style::Color::White),
            style::Print(value),
            Print("\n"),
        )
        .unwrap();
    }

    queue!(stdout, Print("\n"), cursor::MoveToColumn(0)).unwrap();

    stdout.flush().unwrap();
}
