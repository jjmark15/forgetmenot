use assert_fs::prelude::*;
use predicates::str::starts_with;

use crate::helpers::models::config_with_test_without_description;
use crate::helpers::{
    prefix_with_discovered_config, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME,
    DEFAULT_TEST_COMMAND, DEFAULT_TEST_NAME,
};

#[test]
fn describes_test_without_description() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&config_with_test_without_description(), &config_path)
        .unwrap();

    let cmd = CliCommandBuilder::describe_test(DEFAULT_TEST_NAME).with_config(config_path);

    cmd.assert()
        .success()
        .stdout(starts_with(prefix_with_discovered_config(
            format!(
                "\u{1b}[93mname\u{1b}[0m: {}\n\u{1b}[93mcommand\u{1b}[0m: {}",
                DEFAULT_TEST_NAME, DEFAULT_TEST_COMMAND
            ),
            DEFAULT_PROJECT_NAME,
        )));
}
