use std::path::{Path, PathBuf};

use assert_fs::fixture::PathChild;
use predicates::str::starts_with;

use crate::helpers::models::basic_config;
use crate::helpers::{
    create_child_directories_n_levels_deep, prefix_with_discovered_config,
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder, TestDirectoryManager,
    DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME, DEFAULT_TEST_NAME,
};

#[test]
fn accepts_relative_config_path_in_same_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), &config_path).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(test_directory_manager.test_directory())
        .with_config(config_path.file_name().unwrap());

    cmd.assert()
        .success()
        .stdout(starts_with(prefix_with_discovered_config(
            "",
            DEFAULT_PROJECT_NAME,
        )));
}

#[test]
fn accepts_relative_config_path_in_child_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let nested_directory =
        create_child_directories_n_levels_deep(&test_directory_manager.test_directory(), 1);
    let config_path = nested_directory.child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), &config_path).unwrap();
    let relative_config_path: PathBuf = Path::new(".")
        .join("nested")
        .join(DEFAULT_STATED_CONFIG_FILENAME);

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(test_directory_manager.test_directory())
        .with_config(relative_config_path);

    cmd.assert()
        .success()
        .stdout(starts_with(prefix_with_discovered_config("", "nested")));
}
