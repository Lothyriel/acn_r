use std::env;
use anyhow::{anyhow, Error};

pub fn get(var_name: &str) -> Result<String, Error> {
    env::var(var_name).map_err(|_| anyhow!("{} não definido nas variáveis de ambiente", var_name))
}