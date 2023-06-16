use std::sync::Arc;

use anyhow::Error;

use crate::features::events::handlers::voice_updated::DispatchData;

pub async fn handler(_dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    Ok(())
}
