use crate::extensions::log_ext::LogExt;

mod acn;
mod listener;

pub async fn start_acn() {
    acn::start_acn().await.log();
}

pub async fn start_listener() {
    listener::start_listener().await.log();
}