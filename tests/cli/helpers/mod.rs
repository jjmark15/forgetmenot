use std::path::Path;

use owo_colors::OwoColorize;
use predicates::str::{starts_with, StartsWithPredicate};

pub(crate) use cmd_builder::*;
pub(crate) use test_directory::*;

use crate::helpers::models::ApplicationConfig;

mod cmd_builder;
pub(crate) mod formatting;
pub(crate) mod git;
pub(crate) mod models;
mod test_directory;

pub(crate) const AUTO_DISCOVERED_CONFIG_FILENAME: &str = "forgetmenot.yml";
pub(crate) const DEFAULT_STATED_CONFIG_FILENAME: &str = "config.yml";
pub(crate) const DEFAULT_TEST_COMMAND: &str = "echo result";
pub(crate) const DEFAULT_TEST_NAME: &str = "command";
pub(crate) const ALTERNATE_TEST_NAME: &str = "other command";
pub(crate) const DEFAULT_TEST_DESCRIPTION: &str = "test description";
pub(crate) const ALTERNATE_TEST_DESCRIPTION: &str = "test description";
pub(crate) const APPLICATION_NAME: &str = "forgetmenot";
pub(crate) const DEFAULT_PROJECT_NAME: &str = "project";

pub(crate) fn write_application_config_to_file<P: AsRef<Path>>(
    application_config: &ApplicationConfig,
    path: P,
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

pub(crate) fn prefix_with_discovered_config<S: AsRef<str>>(
    string: S,
    config_directory_name: &str,
) -> String {
    format!(
        "Discovered {} config\n\n{}",
        config_directory_name.bright_purple(),
        string.as_ref()
    )
}
