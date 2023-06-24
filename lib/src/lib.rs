use anyhow::Error;

use crate::application::infra::{appsettings, appsettings::TestSettings};

pub mod application;
pub mod extensions;

pub fn get_test_settings() -> Result<TestSettings, Error> {
    appsettings::load()
}
