use std::io::{self, Write};
use kalosm::language::*;

#[tokio::main]
async fn main() {
    let model = Llama::phi_3().await.unwrap();

    // User input via stdin
    print!("Please enter a prompt: ");

    io::stdout().flush().unwrap(); // Make sure the prompt is printed
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim();

    println!("The input: {}", user_input);

    let prompt = format!("{}", user_input);

    // Generate the response 
    let mut stream = model(&prompt);
    stream.to_std_out().await.unwrap();
}
