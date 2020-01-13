use ansi_term::Colour::{Purple, White, Yellow};
use ansi_term::Style;

#[derive(Clone)]
pub struct Peers {
    pub local: String,
    pub remote: String,
}

impl Peers {
    pub fn new(local: String, remote: String) -> Self {
        Peers { local, remote }
    }

    pub fn display_remote(&self) -> String {
        format!(
            "{}{}{}",
            Style::new().fg(White).paint("["),
            Style::new().bold().fg(Purple).paint(&self.local),
            Style::new().fg(White).paint("]"),
        )
    }
}
