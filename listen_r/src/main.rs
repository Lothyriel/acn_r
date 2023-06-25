#[tokio::main]
async fn main() {
    lib::configurations::start_listener().await
}
