use crate::domain::command::TestCommand;
use crate::domain::TestResult;

pub(crate) trait CommandExecutor {
    fn execute(&self, command: &TestCommand) -> Result<TestResult, ExecuteCommandError>;
}

#[derive(Debug, thiserror::Error)]
#[error("failed to execute command")]
pub(crate) struct ExecuteCommandError;
