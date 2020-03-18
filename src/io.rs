use crossterm::terminal::size;

#[derive(Clone)]
pub struct Line {
    pub content: String,
    pub length: u8,
}

impl Line {
    pub fn new(content: String) -> Self {
        let length = Self::get_content_lines_length(content.clone());

        Line { content, length }
    }

    pub fn get_content_lines_length(content: String) -> u8 {
        let content_length = content.clone().len() as f64;
        let terminal_width = size().unwrap().0 as f64;

        (content_length / terminal_width).ceil() as u8
    }
}
