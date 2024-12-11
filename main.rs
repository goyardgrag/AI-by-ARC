use rig::{completion::Prompt, providers::openai};
use egg_mode::{Token, tweet::DraftTweet};
use std::io::{self, Write};  // Import for reading user input

#[tokio::main]
async fn main() {
    // Create the OpenAI client for GPT-4
    let openai_client = openai::Client::new("");
    let gpt4 = openai_client.agent("gpt-4").build();

    // Prompt the user for input
    print!("Enter your prompt: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input

    // Read the user's input from the console
    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt).unwrap();
    let prompt = prompt.trim(); // Remove any trailing newline characters

    // Generate a response from GPT-4 using the user's prompt
    let response = gpt4
        .prompt(prompt)
        .await
        .expect("Error generating the response");

    println!("GPT-4 Generated Tweet: {}", response);

    // Twitter API credentials
    let consumer_key = "";
    let consumer_secret = "";
    let access_token = "";
    let access_token_secret = "";

    let token = Token::Access {
        consumer: egg_mode::KeyPair::new(consumer_key, consumer_secret),
        access: egg_mode::KeyPair::new(access_token, access_token_secret),
    };

    // Send the generated tweet to Twitter
    let draft = DraftTweet::new(response).send(&token).await.unwrap();

    println!("Tweet successfully sent to Twitter!");
}
