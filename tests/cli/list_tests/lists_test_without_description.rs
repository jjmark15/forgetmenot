use assert_fs::fixture::PathChild;
use predicates::str::starts_with;

use crate::helpers::cmd_output::{ListTestsLine, ListTestsOutput};
use crate::helpers::models::config_with_test_without_description;
use crate::helpers::{
    prefix_with_discovered_config, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME,
    DEFAULT_TEST_NAME,
};

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
            ListTestsOutput::single(ListTestsLine::without_description(DEFAULT_TEST_NAME)),
            DEFAULT_PROJECT_NAME,
        )));
}
