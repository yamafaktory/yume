use crossterm::terminal::size;

pub struct Line {
    pub content: String,
    pub length: u8,
}

impl Line {
    pub fn new(content: String) -> Self {
        let content_length = content.clone().len() as f64;
        let terminal_width = size().unwrap().0 as f64;
        let length = (content_length / terminal_width).ceil() as u8;

        Line { content, length }
    }
}
