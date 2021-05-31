use std::process::Command;

use crate::domain::{CommandExecutor, ExecuteCommandError, TestCommand, TestResult};

pub(crate) struct SystemProcessCommandExecutorAdapter {}

impl SystemProcessCommandExecutorAdapter {
    pub(crate) fn new() -> Self {
        SystemProcessCommandExecutorAdapter {}
    }
}

impl CommandExecutor for SystemProcessCommandExecutorAdapter {
    fn execute(&self, command: &TestCommand) -> Result<TestResult, ExecuteCommandError> {
        let output_result = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", command.string()]).output()
        } else {
            Command::new("sh").arg("-c").arg(command.string()).output()
        };
        match output_result {
            Ok(output) => Ok(TestResult::new(
                String::from_utf8_lossy(&output.stdout).to_string(),
                output.status.code().unwrap_or(0),
            )),
            Err(_e) => Err(ExecuteCommandError),
        }
    }
}
