use assert_fs::prelude::*;

use crate::helpers::models::basic_config;
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder,
    DEFAULT_CONFIG_FILENAME, DEFAULT_TEST_NAME,
};

#[test]
fn discovers_config_in_current_directory() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory
        .child(DEFAULT_CONFIG_FILENAME)
        .to_path_buf();
    write_application_config_to_file(&basic_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(temp_home_directory.path());

    cmd.assert().success();
}
