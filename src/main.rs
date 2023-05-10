use acn_r::extensions::log_ext::LogExt;

#[tokio::main]
async fn main() {
    acn_r::start_application().await.log()
}
