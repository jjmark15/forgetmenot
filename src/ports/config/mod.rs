use std::path::Path;

use crate::application::ApplicationTest;

pub(crate) mod file;

pub(crate) trait ConfigReader {
    fn read(&self, config_path: &Path) -> Result<ApplicationConfig, ReadConfigError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReadConfigError {
    #[error(transparent)]
    NotFound(#[from] OpenConfigError),
    #[error(transparent)]
    BadConfig(#[from] BadConfigError),
}

#[derive(Debug, thiserror::Error, Default)]
#[error("could not open config file")]
pub(crate) struct OpenConfigError;

#[derive(Debug, thiserror::Error, Default)]
#[error("config is bad")]
pub(crate) struct BadConfigError;

#[derive(derive_new::new, derive_getters::Getters)]
pub(crate) struct ApplicationConfig {
    tests: Vec<ApplicationTest>,
}
