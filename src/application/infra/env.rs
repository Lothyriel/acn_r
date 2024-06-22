use anyhow::{anyhow, Result};
use env_logger::Target;
use log::LevelFilter;
use std::env;

pub fn get(var_name: &str) -> Result<String> {
    env::var(var_name).map_err(|_| anyhow!("ENV variable {} not defined", var_name))
}

pub fn init() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::builder()
        .filter_level(LevelFilter::Warn)
        .target(Target::Stdout)
        .try_init()?;

    Ok(())
}
