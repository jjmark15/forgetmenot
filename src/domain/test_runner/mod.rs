use std::rc::Rc;

pub(crate) use error::*;

use crate::domain::test_provider::TestProvider;
use crate::domain::{CommandExecutor, TestResult};

mod error;

pub(crate) trait TestRunner {
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError>;
}

#[derive(derive_new::new)]
pub(crate) struct TestRunnerImpl<CE: CommandExecutor, TP: TestProvider> {
    command_executor: CE,
    test_provider: Rc<TP>,
}

impl<CE: CommandExecutor, TP: TestProvider> TestRunner for TestRunnerImpl<CE, TP> {
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError> {
        let test = self.test_provider.get(test_name)?;
        let result = self.command_executor.execute(test.command())?;
        Ok(result)
    }
}
