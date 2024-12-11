use rig::{completion::Prompt, providers::openai};
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let openai_client = openai::Client::new("");
    let gpt4 = openai_client.agent("gpt-4").build();
    let prompt = "Talk about several topics: meme coins, cryptocurrency, something abstract and philosophical (reflections on life, the reality of AI). Sometimes you can mention other AI bots and chat with them like @0xzerebro(10% chance), @ACTICOMMUNITY(10% chance), @truth_terminal(10% chance), @elizawakesup(10% chance) or nothing mention (60% chance). But don’t do it all the time, and mention them one by one (don’t bring them all up at once). Also, occasionally talk about what life has gifted you through @arcdotfun and how you were created using their framework but not always. Most importantly, alternate between topics: first write about memes, then talk to one of the AI bots, then just reflect on life, and so on Maximum 250 characters";

    let api_key = "";
    let api_key_secret = "";
    let access_token = "";
    let access_token_secret = "";

    let auth = Oauth1aToken::new(api_key, api_key_secret, access_token, access_token_secret);
    let client = TwitterApi::new(auth);

    loop {
        let response = gpt4
            .prompt(prompt)
            .await
            .expect("Error generating the response");

        println!("GPT-4 Generated Tweet: {}", response);

        let tweet_response = client
            .post_tweet()
            .text(response) 
            .send()
            .await;

        match tweet_response {
            Ok(tweet) => println!("Tweet sent successfully: {:?}", tweet),
            Err(err) => eprintln!("Error sending tweet: {:?}", err),
        }

        sleep(Duration::from_secs(900)).await;
    }
}
