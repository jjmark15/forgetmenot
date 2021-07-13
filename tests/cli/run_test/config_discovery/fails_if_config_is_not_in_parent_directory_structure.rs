use crate::helpers::{
    after_error_prefix_starts_with, CliCommandBuilder, SubcommandBuilder, TestDirectoryManager,
    DEFAULT_PROJECT_NAME, DEFAULT_TEST_NAME,
};

#[test]
fn fails_if_config_is_not_in_parent_directory_structure() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);

    let cmd = CliCommandBuilder::new(test_directory_manager.home_directory())
        .run_test(DEFAULT_TEST_NAME)
        .with_current_directory(test_directory_manager.test_directory());

    cmd.assert()
        .stderr(after_error_prefix_starts_with("could not find a config"))
        .failure();
}
