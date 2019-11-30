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

            line.clear();

            Ok(input)
        }
        Err(_) => Err(String::from("Can't read stdin!")),
    }
}
