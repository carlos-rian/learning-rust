async fn app() {
    print!("OK")
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = app();
    rt.block_on(future);
}