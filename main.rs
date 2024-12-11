use rig::{completion::Prompt, providers::openai};
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let openai_client = openai::Client::new("");
    let gpt4 = openai_client.agent("gpt-4").build();
    print!("Enter your prompt: ");
    io::stdout().flush().unwrap();
    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt).unwrap();
    let prompt = prompt.trim();

    let response = gpt4
        .prompt(prompt)
        .await
        .expect("Error generating the response");

    println!("GPT-4 Generated Tweet: {}", response);

    let api_key = ""; 
    let api_key_secret = ""; 
    let access_token = "";
    let access_token_secret = "";

    let auth = Oauth1aToken::new(api_key, api_key_secret, access_token, access_token_secret);
    let client = TwitterApi::new(auth);
    let tweet_response = client
        .post_tweet()
        .text(response) 
        .send()
        .await;

    match tweet_response {
        Ok(tweet) => println!("Tweet sent successfully: {:?}", tweet),
        Err(err) => eprintln!("Error sending tweet: {:?}", err),
    }
}
