#[tokio::main]
async fn main() {
    lib::configurations::start_acn().await
}
