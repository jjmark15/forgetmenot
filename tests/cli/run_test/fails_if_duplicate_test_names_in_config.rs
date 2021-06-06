use assert_fs::prelude::PathChild;

use crate::helpers::models::config_with_duplicate_test_name;
use crate::helpers::{
    after_error_prefix_starts_with, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, DEFAULT_TEST_NAME,
};

#[test]
fn fails_if_duplicate_test_names_in_config() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(&config_with_duplicate_test_name(), config_path.as_path())
        .unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME).with_config(config_path.as_path());

    cmd.assert()
        .stderr(after_error_prefix_starts_with(
            "test with name 'command' already exists",
        ))
        .failure();
}
