use assert_fs::prelude::*;

use crate::helpers::models::config_with_no_test;
use crate::helpers::{
    after_error_prefix_starts_with, write_application_config_to_file, CliCommandBuilder,
};

#[test]
fn fails_if_test_does_not_exist() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(&config_with_no_test(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::new().run_test_command(config_path.as_path());

    cmd.assert()
        .stderr(after_error_prefix_starts_with("test 'command' not found"))
        .failure();
}
