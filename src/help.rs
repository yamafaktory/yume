use crate::terminal::println;

const HELP: &'static str = "/help display help";
const QUIT: &'static str = "/quit quit application";

pub async fn render() {
    println(String::from(""), true);

    for line in vec![HELP, QUIT] {
        println(String::from(line), true);
    }

    println(String::from(""), true);
}
