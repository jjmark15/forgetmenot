use crate::domain::command::TestCommand;
use crate::domain::TestResult;

pub(crate) trait CommandExecutor {
    fn execute(&self, command: &TestCommand) -> TestResult;
}
