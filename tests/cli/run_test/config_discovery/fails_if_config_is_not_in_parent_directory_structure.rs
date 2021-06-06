use crate::helpers::{after_error_prefix_starts_with, CliCommandBuilder};

#[test]
fn fails_if_config_is_not_in_parent_directory_structure() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();

    let cmd = CliCommandBuilder::new()
        .with_current_dir(temp_home_directory.path())
        .run_test_command();

    cmd.assert()
        .stderr(after_error_prefix_starts_with("could not find a config"))
        .failure();
}
