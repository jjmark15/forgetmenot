use assert_fs::prelude::*;
use predicates::str::starts_with;

use crate::helpers::git::git_repo_with_single_commit;
use crate::helpers::models::basic_config;
use crate::helpers::{
    prefix_with_discovered_config, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, DEFAULT_PROJECT_NAME, DEFAULT_STATED_CONFIG_FILENAME,
    DEFAULT_TEST_NAME,
};

#[test]
fn runs_command_defined_in_config() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    git_repo_with_single_commit(test_directory_manager.test_directory().path());
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&basic_config(), &config_path).unwrap();

    let cmd = CliCommandBuilder::new(test_directory_manager.home_directory())
        .run_test(DEFAULT_TEST_NAME)
        .with_config(config_path);

    cmd.assert()
        .success()
        .stdout(starts_with(prefix_with_discovered_config(
            "",
            DEFAULT_PROJECT_NAME,
        )));
}
