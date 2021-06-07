use std::path::Path;

use predicates::str::{starts_with, StartsWithPredicate};

pub(crate) use cmd_builder::*;

use crate::helpers::models::ApplicationConfig;

mod cmd_builder;
pub(crate) mod models;

pub(crate) const DEFAULT_CONFIG_FILENAME: &str = "forgetmenot.yml";
pub(crate) const DEFAULT_TEST_NAME: &str = "command";
pub(crate) const DEFAULT_ALTERNATE_TEST_NAME: &str = "other command";

pub(crate) fn write_application_config_to_file(
    application_config: &ApplicationConfig,
    path: &Path,
) -> std::io::Result<()> {
    std::fs::write(path, serde_yaml::to_string(application_config).unwrap())
}

pub(crate) fn after_error_prefix_starts_with<P>(pattern: P) -> StartsWithPredicate
where
    P: Into<String>,
{
    let string = format!(
        "\u{1b}[1m\u{1b}[91merror\u{1b}[0m\u{1b}[0m: {}",
        pattern.into()
    );
    starts_with(string)
}
