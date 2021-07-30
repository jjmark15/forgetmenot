use assert_fs::fixture::PathChild;

use crate::helpers::models::basic_config;
use crate::helpers::{
    create_child_directories_n_levels_deep, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, AUTO_DISCOVERED_CONFIG_FILENAME, DEFAULT_PROJECT_NAME,
    DEFAULT_TEST_NAME,
};

#[test]
fn discovers_config_in_1_parent_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let nested_directory =
        create_child_directories_n_levels_deep(&test_directory_manager.test_directory(), 1);
    let config_path = test_directory_manager
        .test_directory()
        .child(AUTO_DISCOVERED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), config_path).unwrap();

    let cmd = CliCommandBuilder::describe_test(DEFAULT_TEST_NAME)
        .with_current_directory(nested_directory.path());

    cmd.assert().success();
}

#[test]
fn discovers_config_in_2_parent_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let nested_directory =
        create_child_directories_n_levels_deep(&test_directory_manager.test_directory(), 2);
    let config_path = test_directory_manager
        .test_directory()
        .child(AUTO_DISCOVERED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), config_path).unwrap();

    let cmd = CliCommandBuilder::describe_test(DEFAULT_TEST_NAME)
        .with_current_directory(nested_directory.path());

    cmd.assert().success();
}

#[test]
fn discovers_config_in_3_parent_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let nested_directory =
        create_child_directories_n_levels_deep(&test_directory_manager.test_directory(), 3);
    let config_path = test_directory_manager
        .test_directory()
        .child(AUTO_DISCOVERED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), config_path).unwrap();

    let cmd = CliCommandBuilder::describe_test(DEFAULT_TEST_NAME)
        .with_current_directory(nested_directory.path());

    cmd.assert().success();
}
