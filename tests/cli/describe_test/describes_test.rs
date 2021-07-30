use assert_fs::prelude::*;

use crate::helpers::cmd_output::DescribeTestOutput;
use crate::helpers::models::basic_config;
use crate::helpers::{
    prefix_with_discovered_config, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME,
    DEFAULT_TEST_COMMAND, DEFAULT_TEST_DESCRIPTION, DEFAULT_TEST_NAME,
};

#[test]
fn describes_test() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), &config_path).unwrap();

    let cmd = CliCommandBuilder::describe_test(DEFAULT_TEST_NAME).with_config(config_path);

    cmd.assert().success().stdout(prefix_with_discovered_config(
        DescribeTestOutput::new(
            DEFAULT_TEST_NAME,
            DEFAULT_TEST_DESCRIPTION,
            DEFAULT_TEST_COMMAND,
        ),
        DEFAULT_PROJECT_NAME,
    ));
}
