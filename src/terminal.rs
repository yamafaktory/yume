use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal,
};
use std::io::{stdout, Write};

pub fn enter_secondary_screen() {
    terminal::enable_raw_mode().unwrap();

    execute!(stdout(), terminal::EnterAlternateScreen,).unwrap();
}

pub fn println(line: String) {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0),
        Print(format!("{}\n", line)),
        cursor::MoveToColumn(0),
    )
    .unwrap();
}

pub fn prompt(question: Option<String>) -> Result<String, String> {
    let mut characters = String::new();

    if let Some(text) = question {
        println(text);
    }

    while let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
        match code {
            KeyCode::Enter => {
                break;
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
    Ok(characters)
}
