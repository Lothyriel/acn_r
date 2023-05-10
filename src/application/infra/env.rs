use anyhow::{anyhow, Error};
use env_logger::Target;
use log::LevelFilter;
use std::env;

pub fn get(var_name: &str) -> Result<String, Error> {
    env::var(var_name).map_err(|_| anyhow!("ENV variable {} not defined", var_name))
}
