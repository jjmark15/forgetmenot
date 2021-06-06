use crate::helpers::{
    after_error_prefix_starts_with, CliCommandBuilder, SubcommandBuilder, DEFAULT_TEST_NAME,
};

#[test]
fn fails_if_config_is_not_in_parent_directory_structure() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();

    let cmd = CliCommandBuilder::run_test(DEFAULT_TEST_NAME)
        .with_current_directory(temp_home_directory.path());

    cmd.assert()
        .stderr(after_error_prefix_starts_with("could not find a config"))
        .failure();
}
