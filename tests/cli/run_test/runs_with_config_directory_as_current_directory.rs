use assert_fs::prelude::*;

use crate::helpers::models::{ApplicationConfig, TestCommand};
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder, TestDirectoryManager,
    DEFAULT_NESTED_DIRECTORY_NAME, DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME,
    DEFAULT_TEST_NAME,
};

#[test]
fn runs_with_config_directory_as_current_directory() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    let child_directory = test_directory_manager
        .test_directory()
        .child(DEFAULT_NESTED_DIRECTORY_NAME);
    child_directory.create_dir_all().unwrap();
    let config = ApplicationConfig::new(vec![TestCommand::new(
        DEFAULT_TEST_NAME.to_string(),
        touch_command().to_string(),
        None,
    )]);
    write_application_config_to_file(&config, &config_path).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(child_directory)
        .with_config(config_path);

    cmd.assert().success();

    assert!(test_directory_manager
        .test_directory()
        .child("file.txt")
        .exists());
}

fn touch_command() -> &'static str {
    if cfg!(target_os = "windows") {
        "echo $null > file.txt"
    } else {
        "touch file.txt"
    }
}
