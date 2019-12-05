use async_std::io;

pub async fn prompt(question: Option<String>) -> Result<String, String> {
    let stdin = io::stdin();
    let mut line = String::new();

    if let Some(content) = question {
        println!("{}", content);
    }

    match stdin.read_line(&mut line).await {
        Ok(_) => {
            let input = line.clone();
            let mut lines = input.lines();

            line.clear();

            Ok(lines.next().unwrap().to_string())
        }
        Err(_) => Err(String::from("Can't read stdin!")),
    }
}
