pub(crate) use application_config::*;

use crate::helpers::{
    ALTERNATE_TEST_DESCRIPTION, ALTERNATE_TEST_NAME, DEFAULT_TEST_COMMAND,
    DEFAULT_TEST_DESCRIPTION, DEFAULT_TEST_NAME,
};

mod application_config;

pub(crate) fn basic_config() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::new(
        DEFAULT_TEST_NAME.to_string(),
        DEFAULT_TEST_COMMAND.to_string(),
        Some(DEFAULT_TEST_DESCRIPTION.to_string()),
    )])
}

pub(crate) fn failing_test_config() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::new(
        DEFAULT_TEST_NAME.to_string(),
        "./missing-command".to_string(),
        Some(DEFAULT_TEST_DESCRIPTION.to_string()),
    )])
}

pub(crate) fn config_with_no_test() -> ApplicationConfig {
    ApplicationConfig::new(vec![])
}

pub(crate) fn config_with_duplicate_test_name() -> ApplicationConfig {
    ApplicationConfig::new(vec![
        TestCommand::new(
            DEFAULT_TEST_NAME.to_string(),
            DEFAULT_TEST_COMMAND.to_string(),
            Some(DEFAULT_TEST_DESCRIPTION.to_string()),
        ),
        TestCommand::new(
            DEFAULT_TEST_NAME.to_string(),
            DEFAULT_TEST_COMMAND.to_string(),
            Some(DEFAULT_TEST_DESCRIPTION.to_string()),
        ),
    ])
}

pub(crate) fn config_with_multiple_tests() -> ApplicationConfig {
    ApplicationConfig::new(vec![
        TestCommand::new(
            DEFAULT_TEST_NAME.to_string(),
            DEFAULT_TEST_COMMAND.to_string(),
            Some(DEFAULT_TEST_DESCRIPTION.to_string()),
        ),
        TestCommand::new(
            ALTERNATE_TEST_NAME.to_string(),
            DEFAULT_TEST_COMMAND.to_string(),
            Some(ALTERNATE_TEST_DESCRIPTION.to_string()),
        ),
    ])
}

pub(crate) fn config_with_test_without_description() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::new(
        DEFAULT_TEST_NAME.to_string(),
        DEFAULT_TEST_COMMAND.to_string(),
        None,
    )])
}
