pub(crate) use application_config::*;

use crate::helpers::{DEFAULT_ALTERNATE_TEST_NAME, DEFAULT_TEST_NAME};

mod application_config;

pub(crate) fn basic_config() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::new(
        DEFAULT_TEST_NAME.to_string(),
        "echo result".to_string(),
    )])
}

pub(crate) fn failing_test_config() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::new(
        DEFAULT_TEST_NAME.to_string(),
        "./missing-command".to_string(),
    )])
}

pub(crate) fn config_with_no_test() -> ApplicationConfig {
    ApplicationConfig::new(vec![])
}

pub(crate) fn config_with_duplicate_test_name() -> ApplicationConfig {
    ApplicationConfig::new(vec![
        TestCommand::new(DEFAULT_TEST_NAME.to_string(), "echo result".to_string()),
        TestCommand::new(DEFAULT_TEST_NAME.to_string(), "echo result".to_string()),
    ])
}

pub(crate) fn config_with_multiple_tests() -> ApplicationConfig {
    ApplicationConfig::new(vec![
        TestCommand::new(DEFAULT_TEST_NAME.to_string(), "echo result".to_string()),
        TestCommand::new(
            DEFAULT_ALTERNATE_TEST_NAME.to_string(),
            "echo result".to_string(),
        ),
    ])
}
