use std::rc::Rc;

pub(crate) use test::*;

use crate::application::error::RunTestError;
use crate::application::test_result::TestResult;
use crate::domain::{TestProvider, TestRunner};

mod error;
mod test;
mod test_result;

pub(crate) trait ApplicationService {
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError>;

    fn list_tests(&self) -> Vec<ApplicationTest>;
}

pub(crate) struct ApplicationServiceImpl<TR: TestRunner, TP: TestProvider> {
    test_runner: TR,
    test_provider: Rc<TP>,
}

impl<TR, TP> ApplicationServiceImpl<TR, TP>
where
    TR: TestRunner,
    TP: TestProvider,
{
    pub(crate) fn new(test_runner: TR, test_provider: Rc<TP>) -> Self {
        ApplicationServiceImpl {
            test_runner,
            test_provider,
        }
    }
}

impl<TR, TP> ApplicationService for ApplicationServiceImpl<TR, TP>
where
    TR: TestRunner,
    TP: TestProvider,
{
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError> {
        let test_result: TestResult = self.test_runner.run_test(test_name)?.into();
        Ok(test_result)
    }

    fn list_tests(&self) -> Vec<ApplicationTest> {
        self.test_provider
            .get_all()
            .into_iter()
            .map(ApplicationTest::from)
            .collect()
    }
}
