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

pub(crate) struct ApplicationConfig {
    tests: Vec<ApplicationTest>,
}

impl ApplicationConfig {
    pub(crate) fn new(tests: Vec<ApplicationTest>) -> Self {
        ApplicationConfig { tests }
    }

    pub(crate) fn tests(&self) -> &Vec<ApplicationTest> {
        &self.tests
    }
}
