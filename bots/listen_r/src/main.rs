use lib::extensions::log_ext::LogExt;

#[tokio::main]
async fn main() {
    lib::start_listener().await.log()
}