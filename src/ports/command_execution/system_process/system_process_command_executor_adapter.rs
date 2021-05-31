use std::process::Command;

use crate::domain::{CommandExecutor, TestCommand, TestResult};

pub(crate) struct SystemProcessCommandExecutorAdapter {}

impl SystemProcessCommandExecutorAdapter {
    pub(crate) fn new() -> Self {
        SystemProcessCommandExecutorAdapter {}
    }
}

impl CommandExecutor for SystemProcessCommandExecutorAdapter {
    fn execute(&self, command: &TestCommand) -> TestResult {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", command.string()])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command.string())
                .output()
                .expect("failed to execute process")
        };
        TestResult::new(
            String::from_utf8_lossy(&output.stdout).to_string(),
            output.status.code().unwrap_or(0),
        )
    }
}
