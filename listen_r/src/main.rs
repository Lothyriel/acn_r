#[tokio::main]
async fn main() {
    lib::features::start_listener().await
}
