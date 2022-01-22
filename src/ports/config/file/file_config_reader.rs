use std::fs::File;
use std::path::Path;

use crate::ports::config::file::application_config::SerdeApplicationConfig;
use crate::ports::config::{
    ApplicationConfig, BadConfigError, ConfigReader, OpenConfigError, ReadConfigError,
};

#[derive(derive_new::new)]
pub(crate) struct FileConfigReader {}

impl ConfigReader for FileConfigReader {
    fn read(&self, config_path: &Path) -> Result<ApplicationConfig, ReadConfigError> {
        let file = File::open(config_path).map_err(|_e| OpenConfigError::default())?;
        let config: SerdeApplicationConfig =
            serde_yaml::from_reader(file).map_err(|_e| BadConfigError::default())?;
        Ok(config.into())
    }
}
