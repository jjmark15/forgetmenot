use std::fmt::Display;

use assert_fs::fixture::PathChild;

use crate::helpers::models::{basic_config, config_with_multiple_tests};
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder,
    DEFAULT_ALTERNATE_TEST_NAME, DEFAULT_TEST_NAME,
};

#[test]
fn lists_single_test() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(&basic_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path.as_path());

    cmd.assert()
        .success()
        .stdout(format!("{}\n", green(DEFAULT_TEST_NAME)));
}

#[test]
fn lists_multiple_tests_in_alphabetical_order() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(&config_with_multiple_tests(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path.as_path());

    cmd.assert().success().stdout(format!(
        "{}\n{}\n",
        green(DEFAULT_TEST_NAME),
        green(DEFAULT_ALTERNATE_TEST_NAME)
    ));
}

fn green<S: AsRef<str> + Display>(string: S) -> String {
    format!("\u{1b}[92m{}\u{1b}[0m", string)
}
