use std::path::Path;

pub(crate) use cmd_builder::*;

use crate::helpers::models::ApplicationConfig;

mod cmd_builder;
pub(crate) mod models;

pub(crate) fn write_application_config_to_file(
    application_config: &ApplicationConfig,
    path: &Path,
) -> std::io::Result<()> {
    std::fs::write(path, serde_yaml::to_string(application_config).unwrap())
}
