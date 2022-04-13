use futures::future;
use tokio::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


async fn our_async_program() -> Result<String> {
    println!("{}", "init");
    let v = future::ok("Hello world".to_string()).await;
    println!("{}", "end");
    v
}

async fn app() {
    let concurrent_future = task::spawn(our_async_program());
    print!("app: {:?}", concurrent_future.await);
}

#[tokio::main]
async fn main() {
    app().await;
}