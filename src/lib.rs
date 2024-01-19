use extensions::log_ext::LogExt;

pub mod application;
pub mod extensions;
pub mod features;

pub async fn start() {
    features::start().await.log()
}
