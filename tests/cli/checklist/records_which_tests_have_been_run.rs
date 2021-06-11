use std::path::Path;

use assert_fs::fixture::PathChild;

use crate::helpers::formatting::yellow;
use crate::helpers::models::config_with_multiple_tests;
use crate::helpers::{
    prefix_with_discovered_config, write_application_config_to_file, CliCommandBuilder,
    SubcommandBuilder, TestDirectoryManager, ALTERNATE_TEST_NAME, DEFAULT_PROJECT_NAME,
    DEFAULT_STATED_CONFIG_FILENAME, DEFAULT_TEST_NAME,
};

#[test]
fn records_which_tests_have_been_run_against_the_latest_vcs_version() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    let test_repo_directory = test_directory_manager.test_directory().child("repo");
    create_test_git_repo(test_repo_directory.path());
    let config_path = test_directory_manager
        .test_directory()
        .child(DEFAULT_STATED_CONFIG_FILENAME);
    write_application_config_to_file(&config_with_multiple_tests(), &config_path).unwrap();
    CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_config(&config_path)
        .assert()
        .success();

    let cmd = CliCommandBuilder::view_checklist().with_config(config_path);

    cmd.assert().success().stdout(prefix_with_discovered_config(
        format!(
            "{}: ✔\n{}: ❌\n",
            yellow(DEFAULT_TEST_NAME),
            yellow(ALTERNATE_TEST_NAME)
        ),
        DEFAULT_PROJECT_NAME,
    ));
}

fn create_test_git_repo(repo_path: &Path) {
    if !repo_path.exists() {
        std::fs::create_dir_all(repo_path).unwrap();
    }
    git2::Repository::init(repo_path).unwrap();
}
