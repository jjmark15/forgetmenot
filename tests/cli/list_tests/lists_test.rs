use assert_fs::fixture::PathChild;

use crate::helpers::cmd_output::{ListTestsLine, ListTestsOutput};
use crate::helpers::models::{basic_config, config_with_multiple_tests};
use crate::helpers::{
    prefix_with_discovered_config, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, ALTERNATE_TEST_DESCRIPTION, ALTERNATE_TEST_NAME,
    DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME, DEFAULT_TEST_DESCRIPTION,
    DEFAULT_TEST_NAME,
};

#[test]
fn lists_single_test() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), &config_path).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path);

    cmd.assert().success().stdout(prefix_with_discovered_config(
        ListTestsOutput::single(ListTestsLine::new(
            DEFAULT_TEST_NAME,
            DEFAULT_TEST_DESCRIPTION,
        )),
        DEFAULT_PROJECT_NAME,
    ));
}

#[test]
fn lists_multiple_tests_in_alphabetical_order() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&config_with_multiple_tests(), &config_path).unwrap();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path);

    cmd.assert().success().stdout(prefix_with_discovered_config(
        ListTestsOutput::new(&[
            ListTestsLine::new(DEFAULT_TEST_NAME, DEFAULT_TEST_DESCRIPTION),
            ListTestsLine::new(ALTERNATE_TEST_NAME, ALTERNATE_TEST_DESCRIPTION),
        ]),
        DEFAULT_PROJECT_NAME,
    ));
}
