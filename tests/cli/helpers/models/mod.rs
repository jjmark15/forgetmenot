pub(crate) use application_config::*;

mod application_config;

pub(crate) fn basic_config() -> ApplicationConfig {
    application_tests_from_list(vec![("command", Test::new("echo result".to_string()))])
}

pub(crate) fn failing_test_config() -> ApplicationConfig {
    application_tests_from_list(vec![(
        "command",
        Test::new("./missing-command".to_string()),
    )])
}

pub(crate) fn config_with_no_test() -> ApplicationConfig {
    application_tests_from_list(vec![])
}

fn application_tests_from_list(list: Vec<(&str, Test)>) -> ApplicationConfig {
    let tests = list
        .into_iter()
        .map(|(name, command)| (name.to_string(), command))
        .collect();

    ApplicationConfig::new(tests)
}
