use assert_cmd::Command;
use predicates::str::starts_with;

#[test]
fn says_hello_world() {
    let mut cmd = Command::cargo_bin("forgetmenot").unwrap();
    cmd.assert().stdout(starts_with("Hello, world!")).success();
}
