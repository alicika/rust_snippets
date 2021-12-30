use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct TweetBody<'a> {
    txt: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let t = reqwest::Client::new();

    let v = std::env::args().nth(1);

    // if there is a searching query given as the first argument,
    // set it as a parameter
    let url = match v {
        None => "https://api.twitter.com/1.1/search/tweets.json".to_string(),
        Some(query) => format!("https://api.twitter.com/1.1/search/tweets.json?{}", query),
    };

    let resp = t
        .get(url)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
