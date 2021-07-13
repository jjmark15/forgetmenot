use predicates::str::is_match;

use crate::helpers::{CliCommandBuilder, TestDirectoryManager, DEFAULT_PROJECT_NAME};

#[test]
fn prints_application_name_with_version() {
    let test_directory_manager = TestDirectoryManager::new(DEFAULT_PROJECT_NAME);
    CliCommandBuilder::new(test_directory_manager.home_directory())
        .version()
        .stdout(is_match(r"forgetmenot \d+\.\d+\.\d+").unwrap())
        .success();
}
