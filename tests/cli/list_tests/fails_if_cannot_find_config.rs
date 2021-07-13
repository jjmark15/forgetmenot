use assert_fs::prelude::*;

use crate::helpers::{
    after_error_prefix_starts_with, CliCommandBuilder, SubcommandBuilder, TestDirectoryManager,
    DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME,
};

#[test]
fn fails_if_cannot_find_config() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME)
        .to_path_buf();

    let cmd = CliCommandBuilder::new(test_directory_manager.home_directory())
        .list_tests()
        .with_config(config_path.as_path());

    cmd.assert()
        .stderr(after_error_prefix_starts_with("could not open config file"))
        .failure();
}
