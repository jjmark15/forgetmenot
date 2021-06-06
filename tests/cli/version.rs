use predicates::str::is_match;

use crate::helpers::CliCommandBuilder;

#[test]
fn prints_application_name_with_version() {
    CliCommandBuilder::version()
        .stdout(is_match(r"forgetmenot \d+\.\d+\.\d+").unwrap())
        .success();
}
