use std::fmt::Display;

use assert_fs::fixture::PathChild;
use predicates::str::starts_with;

use crate::helpers::models::{
    basic_config, config_with_multiple_tests, config_with_test_without_description,
};
use crate::helpers::{
    prefix_with_discovered_config, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, ALTERNATE_TEST_DESCRIPTION, ALTERNATE_TEST_NAME,
    DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME, DEFAULT_TEST_DESCRIPTION,
    DEFAULT_TEST_NAME,
};

#[test]
fn lists_single_test() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), &config_path).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path);

    cmd.assert().success().stdout(prefix_with_discovered_config(
        list_tests_line(DEFAULT_TEST_NAME, DEFAULT_TEST_DESCRIPTION),
        DEFAULT_PROJECT_NAME,
    ));
}

#[test]
fn lists_multiple_tests_in_alphabetical_order() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&config_with_multiple_tests(), &config_path).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path);

    cmd.assert().success().stdout(prefix_with_discovered_config(
        format!(
            "{}{}",
            list_tests_line(DEFAULT_TEST_NAME, DEFAULT_TEST_DESCRIPTION),
            list_tests_line(ALTERNATE_TEST_NAME, ALTERNATE_TEST_DESCRIPTION)
        ),
        DEFAULT_PROJECT_NAME,
    ));
}

#[test]
fn lists_test_that_does_not_have_description() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&config_with_test_without_description(), &config_path)
        .unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path);

    cmd.assert()
        .success()
        .stdout(starts_with(prefix_with_discovered_config(
            green(DEFAULT_TEST_NAME),
            DEFAULT_PROJECT_NAME,
        )));
}

fn list_tests_line(test_name: &str, test_description: &str) -> String {
    format!("{} - {}\n", green(test_name), yellow(test_description))
}

fn green<S: AsRef<str> + Display>(string: S) -> String {
    format!("\u{1b}[92m{}\u{1b}[0m", string)
}

fn yellow<S: AsRef<str> + Display>(string: S) -> String {
    format!("\u{1b}[93m{}\u{1b}[0m", string)
}
