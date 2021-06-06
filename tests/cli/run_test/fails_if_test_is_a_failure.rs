use assert_fs::prelude::*;

use crate::helpers::models::failing_test_config;
use crate::helpers::{write_application_config_to_file, CliCommandBuilder};

#[test]
fn fails_if_test_is_a_failure() {
    let temp_home_directory = assert_fs::TempDir::new().unwrap();
    let config_path = temp_home_directory.child("config.yml").to_path_buf();
    write_application_config_to_file(&failing_test_config(), config_path.as_path()).unwrap();

    let cmd = CliCommandBuilder::new()
        .run_test_command()
        .with_config(config_path.as_path());

    cmd.assert().failure();
}
