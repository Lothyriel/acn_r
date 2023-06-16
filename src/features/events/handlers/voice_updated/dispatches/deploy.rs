use std::sync::Arc;

use anyhow::Error;

use crate::features::events::handlers::voice_updated::DispatchData;

pub async fn handler(dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    let http = dispatch_data.http.to_owned();
    let services = dispatch_data.github_services.to_owned();
    let cache = dispatch_data.cache.to_owned();

    services.try_deploy(http, cache).await
}
