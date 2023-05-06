use acn_r::{extensions::log_ext::LogExt, start_application};

#[tokio::main]
async fn main() {
    start_application().await.log()
}
