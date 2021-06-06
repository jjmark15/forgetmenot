use assert_fs::prelude::*;

use crate::helpers::models::{ApplicationConfig, TestCommand};
use crate::helpers::{
    write_application_config_to_file, CliCommandBuilder, SubcommandBuilder, DEFAULT_TEST_NAME,
};

#[test]
fn runs_with_config_directory_as_current_directory() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let child_directory = temp_home_directory.child("nested");
    child_directory.create_dir_all().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    let config = ApplicationConfig::new(vec![TestCommand::new(
        "command".to_string(),
        touch_command().to_string(),
    )]);
    write_application_config_to_file(&config, config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(child_directory.path())
        .with_config(config_path.as_path());

    cmd.assert().success();

    assert_eq!(true, temp_home_directory.child("file.txt").exists());
}

fn touch_command() -> &'static str {
    if cfg!(target_os = "windows") {
        "echo $null > file.txt"
    } else {
        "touch file.txt"
    }
}
