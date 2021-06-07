use assert_fs::prelude::*;

use crate::helpers::{
    after_error_prefix_starts_with, CliCommandBuilder, SubcommandBuilder, DEFAULT_TEST_NAME,
};

#[test]
fn fails_if_cannot_find_config() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME).with_config(config_path.as_path());

    cmd.assert()
        .stderr(after_error_prefix_starts_with("could not open config file"))
        .failure();
}