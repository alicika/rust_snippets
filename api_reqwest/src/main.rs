use reqwest;
use tokio;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct TweetBody<'a> {
    txt: &'a str
}

#[allow(dead_code)]
struct Tweets<'a> {
    field: Vec<TweetBody<'a>>
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let t = reqwest::Client::new();

    let resp = t
        .get("https://api.twitter.com/1.1/search/tweets.json")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
