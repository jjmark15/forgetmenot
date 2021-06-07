use assert_fs::prelude::*;

use crate::helpers::{
    after_error_prefix_starts_with, CliCommandBuilder, SubcommandBuilder,
    DEFAULT_STATED_CONFIG_FILENAME,
};

#[test]
fn fails_if_cannot_find_config() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory
        .child(DEFAULT_STATED_CONFIG_FILENAME)
        .to_path_buf();

    let cmd = CliCommandBuilder::list_tests().with_config(config_path.as_path());

    cmd.assert()
        .stderr(after_error_prefix_starts_with("could not open config file"))
        .failure();
}
