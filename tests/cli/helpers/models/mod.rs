pub(crate) use application_config::*;

use crate::helpers::{Builder, ALTERNATE_TEST_NAME};

mod application_config;

pub(crate) fn basic_config() -> ApplicationConfig {
    ApplicationConfig::single(TestCommand::default())
}

pub(crate) fn failing_test_config() -> ApplicationConfig {
    ApplicationConfig::single(
        TestCommand::builder()
            .with_command("./missing-command".to_string())
            .build(),
    )
}

pub(crate) fn config_with_no_test() -> ApplicationConfig {
    ApplicationConfig::new(vec![])
}

pub(crate) fn config_with_duplicate_test_name() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::default(), TestCommand::default()])
}

pub(crate) fn config_with_multiple_tests() -> ApplicationConfig {
    ApplicationConfig::new(vec![
        TestCommand::default(),
        TestCommand::builder()
            .with_name(ALTERNATE_TEST_NAME.to_string())
            .build(),
    ])
}

pub(crate) fn config_with_test_without_description() -> ApplicationConfig {
    ApplicationConfig::single(TestCommand::builder().with_description(None).build())
}
