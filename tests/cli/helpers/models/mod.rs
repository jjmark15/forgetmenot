pub(crate) use application_config::*;

mod application_config;

pub(crate) fn basic_config() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::new(
        "command".to_string(),
        "echo result".to_string(),
    )])
}

pub(crate) fn failing_test_config() -> ApplicationConfig {
    ApplicationConfig::new(vec![TestCommand::new(
        "command".to_string(),
        "./missing-command".to_string(),
    )])
}
