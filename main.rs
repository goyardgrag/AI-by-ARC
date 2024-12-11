use rig::{completion::Prompt, providers::openai};
use egg_mode::{Token, tweet::DraftTweet};

#[tokio::main]
async fn main() {

    let openai_client = openai::Client::new("");
    let gpt4 = openai_client.agent("gpt-4").build();

    let response = gpt4
        .prompt("Hello")
        .await
        .expect("Error with generating");

    println!("GPT-4 Generated Tweet: {}", response);
    let consumer_key = "";
    let consumer_secret = "";
    let access_token = "";
    let access_token_secret = "";

    let token = Token::Access {
        consumer: egg_mode::KeyPair::new(consumer_key, consumer_secret),
        access: egg_mode::KeyPair::new(access_token, access_token_secret),
    };

    let draft = DraftTweet::new(response).send(&token).await.unwrap();


}
