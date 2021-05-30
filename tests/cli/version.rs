use predicates::str::is_match;

use crate::helpers::CliCommandBuilder;

#[test]
fn prints_application_name_with_version() {
    let cmd = CliCommandBuilder::new().version();
    cmd.assert()
        .stdout(is_match(r"forgetmenot \d+\.\d+\.\d+").unwrap())
        .success();
}
