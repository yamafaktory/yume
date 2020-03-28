use crate::terminal::println;

const HELP: &'static str = "/help display help";
const QUIT: &'static str = "/quit quit application";

pub async fn render() {
    for line in vec![HELP, QUIT] {
        println(String::from(line), true);
    }
}
