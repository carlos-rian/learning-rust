use reqwest;
use std::{thread, time};
use tokio;
use tokio::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn slowwly(delay_ms: u64) -> reqwest::Url {
    thread::sleep(time::Duration::from_millis(delay_ms));
    let url: &str = "https://viacep.com.br/ws/01001000/json/";
    reqwest::Url::parse(&url).unwrap()
}

async fn request(time: u64) -> Result<()> {
    let v: reqwest::Response = reqwest::get(slowwly(time)).await?;
    println!("Got response {}", time);
    println!("{:?}", v.text_with_charset("utf-8").await);
    Ok(())
}

async fn app() -> Result<String> {
    let resp1 = task::spawn(request(2));
    let resp2 = task::spawn(request(1));

    let _ = resp1.await??;
    let _ = resp2.await??;

    Ok("Ok".to_string())
}
#[tokio::main]
async fn main() {
    let x = app().await;

    println!("{:?}", x)
}
