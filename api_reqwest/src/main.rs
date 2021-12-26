use reqwest;
use tokio::net::TcpStream;

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let t = reqwest::Client::builder().build()?;
    println!("{:?}", t);
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp)
}
