use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use assert_fs::TempDir;

use crate::helpers::models::basic_config;
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder,
    DEFAULT_CONFIG_FILENAME, DEFAULT_TEST_NAME,
};

#[test]
fn discovers_config_in_1_parent_directory() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let child_directory = create_child_directories_n_levels_deep(&temp_home_directory, 1);
    let config_path = temp_home_directory
        .child(DEFAULT_CONFIG_FILENAME)
        .to_path_buf();
    write_application_config_to_file(&basic_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(child_directory.path());

    cmd.assert().success();
}

#[test]
fn discovers_config_in_2_parent_directory() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let child_directory = create_child_directories_n_levels_deep(&temp_home_directory, 2);
    let config_path = temp_home_directory
        .child(DEFAULT_CONFIG_FILENAME)
        .to_path_buf();
    write_application_config_to_file(&basic_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(child_directory.path());

    cmd.assert().success();
}

#[test]
fn discovers_config_in_3_parent_directory() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let child_directory = create_child_directories_n_levels_deep(&temp_home_directory, 3);
    let config_path = temp_home_directory
        .child(DEFAULT_CONFIG_FILENAME)
        .to_path_buf();
    write_application_config_to_file(&basic_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(child_directory.path());

    cmd.assert().success();
}

fn nth_child_directory(starting_directory: &TempDir, n: u8) -> ChildPath {
    let mut directory = starting_directory.child(".");

    for _i in 0..n {
        directory = directory.child("nested");
    }

    directory
}

fn create_child_directories_n_levels_deep(starting_directory: &TempDir, n: u8) -> ChildPath {
    let directory = nth_child_directory(starting_directory, n);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}
