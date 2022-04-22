use assert_fs::prelude::*;
use predicates::str::ends_with;

use crate::helpers::models::{ApplicationConfig, TestCommand};
use crate::helpers::{
    write_application_config_to_file, Builder, CliCommandBuilder, SubcommandBuilder,
    TestDirectoryManager, DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME, DEFAULT_TEST_NAME,
};

#[test]
fn runs_test_with_system_environment_variables() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    let config = ApplicationConfig::single(
        TestCommand::builder()
            .with_command("echo $__FORGETMENOT_TEST_VAR".to_string())
            .build(),
    );
    write_application_config_to_file(&config, &config_path).unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_config(config_path)
        .with_env_var("__FORGETMENOT_TEST_VAR", "VALUE");

    cmd.assert().success().stdout(ends_with("VALUE\n"));
}
