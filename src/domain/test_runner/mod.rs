use std::collections::HashMap;
use std::rc::Rc;

pub(crate) use error::*;

use crate::domain::test_provider::TestProvider;
use crate::domain::{
    CommandExecutor, Test, TestHistory, TestHistoryRepository, TestResult, VcsRepositoryProvider,
};

mod error;

pub(crate) trait TestRunner {
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError>;
}

pub(crate) struct TestRunnerImpl<
    CE: CommandExecutor,
    TP: TestProvider,
    THR: TestHistoryRepository,
    VRP: VcsRepositoryProvider,
> {
    command_executor: CE,
    test_provider: Rc<TP>,
    test_history_repository: Rc<THR>,
    vcs_repository_provider: Rc<VRP>,
}

impl<
        CE: CommandExecutor,
        TP: TestProvider,
        THR: TestHistoryRepository,
        VRP: VcsRepositoryProvider,
    > TestRunnerImpl<CE, TP, THR, VRP>
{
    pub(crate) fn new(
        command_executor: CE,
        test_provider: Rc<TP>,
        test_history_repository: Rc<THR>,
        vcs_repository_provider: Rc<VRP>,
    ) -> Self {
        TestRunnerImpl {
            command_executor,
            test_provider,
            test_history_repository,
            vcs_repository_provider,
        }
    }

    fn update_test_history(
        &self,
        test: &Test,
        test_result: TestResult,
    ) -> Result<(), RunTestError> {
        let repository = self.vcs_repository_provider.get().map_err(|_e| {
            RunTestError::UpdateTestHistory(UpdateTestHistoryError::GetCurrentVersion)
        })?;
        let version = repository.version();
        let mut test_history = self
            .test_history_repository
            .get(test.name())
            .unwrap_or_else(|_| TestHistory::new(HashMap::new()));
        test_history.update_result_for(version.clone(), test_result);
        self.test_history_repository
            .store(test.name(), test_history)
            .map_err(|e| UpdateTestHistoryError::StoreTestHistory(e).into())
    }
}

impl<
        CE: CommandExecutor,
        TP: TestProvider,
        THR: TestHistoryRepository,
        VRP: VcsRepositoryProvider,
    > TestRunner for TestRunnerImpl<CE, TP, THR, VRP>
{
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError> {
        let test = self.test_provider.get(test_name)?;
        let result = self.command_executor.execute(test.command())?;
        self.update_test_history(test, result)?;
        Ok(result)
    }
}
