use assert_fs::fixture::{ChildPath, PathChild};

use crate::helpers::models::basic_config;
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder, TestDirectoryManager,
    DEFAULT_CONFIG_FILENAME, DEFAULT_PROJECT_NAME, DEFAULT_TEST_NAME,
};

#[test]
fn discovers_config_in_1_parent_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let nested_directory =
        create_child_directories_n_levels_deep(&test_directory_manager.test_directory(), 1);
    let config_path = nested_directory.child(DEFAULT_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), config_path).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(nested_directory.path());

    cmd.assert().success();
}

#[test]
fn discovers_config_in_2_parent_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let nested_directory =
        create_child_directories_n_levels_deep(&test_directory_manager.test_directory(), 2);
    let config_path = nested_directory.child(DEFAULT_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), config_path).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(nested_directory.path());

    cmd.assert().success();
}

#[test]
fn discovers_config_in_3_parent_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let nested_directory =
        create_child_directories_n_levels_deep(&test_directory_manager.test_directory(), 3);
    let config_path = nested_directory.child(DEFAULT_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), config_path).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(nested_directory.path());

    cmd.assert().success();
}

fn nth_child_directory(starting_directory: &ChildPath, n: u8) -> ChildPath {
    let mut directory = starting_directory.child(".");

    for _i in 0..n {
        directory = directory.child("nested");
    }

    directory
}

fn create_child_directories_n_levels_deep(starting_directory: &ChildPath, n: u8) -> ChildPath {
    let directory = nth_child_directory(starting_directory, n);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}
