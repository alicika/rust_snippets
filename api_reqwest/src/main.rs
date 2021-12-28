use reqwest;
use tokio;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let t = reqwest::Client::new();
    
    let resp = t.get("https://api.twitter.com/1.1/search/tweets.json")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
