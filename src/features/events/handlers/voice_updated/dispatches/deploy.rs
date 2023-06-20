use std::sync::Arc;

use anyhow::Error;

use crate::features::events::handlers::voice_updated::DispatchData;

pub async fn handler(dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    let services = dispatch_data.deploy_services.to_owned();

    let http = dispatch_data.http.to_owned();

    let cache = dispatch_data.cache.to_owned();

    services.try_deploy(http, cache).await
}
