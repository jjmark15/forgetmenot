use std::path::PathBuf;

use crate::helpers::{after_error_prefix_starts_with, CliCommandBuilder};

#[test]
fn fails_if_duplicate_test_names_in_config() {
    let config_path: PathBuf = test_config_directory("config_with_duplicate_test_names.yml");

    let cmd = CliCommandBuilder::new().run_test_command(config_path.as_path());

    cmd.assert()
        .stderr(after_error_prefix_starts_with(
            "test with name 'command' already exists",
        ))
        .failure();
}

fn test_config_directory(file_name: &str) -> PathBuf {
    let project_directory = std::env::current_dir().unwrap();

    project_directory.join("test_files").join(file_name)
}
