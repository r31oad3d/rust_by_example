async fn hello() {
    println!("HELLO!");
}
#[async_std::main]
async fn main() {
    hello().await;
}
