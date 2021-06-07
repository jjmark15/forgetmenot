use std::fmt::Display;

use assert_fs::fixture::PathChild;
use predicates::str::starts_with;

use crate::helpers::models::{
    basic_config, config_with_multiple_tests, config_with_test_without_description,
};
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder,
    ALTERNATE_TEST_DESCRIPTION, ALTERNATE_TEST_NAME, DEFAULT_TEST_DESCRIPTION, DEFAULT_TEST_NAME,
};

#[test]
fn lists_single_test() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(&basic_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path.as_path());

    cmd.assert()
        .success()
        .stdout(list_tests_line(DEFAULT_TEST_NAME, DEFAULT_TEST_DESCRIPTION));
}

#[test]
fn lists_multiple_tests_in_alphabetical_order() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(&config_with_multiple_tests(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path.as_path());

    cmd.assert().success().stdout(format!(
        "{}{}",
        list_tests_line(DEFAULT_TEST_NAME, DEFAULT_TEST_DESCRIPTION),
        list_tests_line(ALTERNATE_TEST_NAME, ALTERNATE_TEST_DESCRIPTION)
    ));
}

#[test]
fn lists_test_that_does_not_have_description() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(
        &config_with_test_without_description(),
        config_path.as_path(),
    )
    .unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path.as_path());

    cmd.assert()
        .success()
        .stdout(starts_with(green(DEFAULT_TEST_NAME)));
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
