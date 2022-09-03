use std::process::Command;

use derive_new::new;

use crate::domain::{CommandExecutor, ExecuteCommandError, TestCommand, TestResult};

#[derive(new)]
pub(crate) struct ShellProcessCommandExecutor {}

impl CommandExecutor for ShellProcessCommandExecutor {
    fn execute(&self, command: &TestCommand) -> Result<TestResult, ExecuteCommandError> {
        let spawn_result = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(&["-Command", command.string()])
                .envs(std::env::vars())
                .spawn()
        } else {
            let shell: String = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
            Command::new(shell)
                .arg("-c")
                .arg(command.string())
                .envs(std::env::vars())
                .spawn()
        };

        let result = spawn_result.and_then(|mut child| child.wait());

        match result {
            Ok(exit_status) => Ok(TestResult::new(exit_status.code().unwrap_or(1))),
            Err(_e) => Err(ExecuteCommandError),
        }
    }
}
