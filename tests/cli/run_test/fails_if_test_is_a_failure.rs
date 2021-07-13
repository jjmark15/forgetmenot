use assert_fs::prelude::*;

use crate::helpers::models::failing_test_config;
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder, TestDirectoryManager,
    DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME, DEFAULT_TEST_NAME,
};

#[test]
fn fails_if_test_is_a_failure() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME)
        .to_path_buf();
    write_application_config_to_file(&failing_test_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::new(test_directory_manager.home_directory())
        .run_test(DEFAULT_TEST_NAME)
        .with_config(config_path.as_path());

    cmd.assert().failure();
}
