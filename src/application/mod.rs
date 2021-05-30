pub(crate) use test::*;

use crate::application::error::RunTestError;
use crate::application::test_result::TestResult;
use crate::domain::TestRunner;

mod error;
mod test;
mod test_result;

pub(crate) trait ApplicationService {
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError>;
}

pub(crate) struct ApplicationServiceImpl<TR: TestRunner> {
    test_runner: TR,
}

impl<TR: TestRunner> ApplicationServiceImpl<TR> {
    pub(crate) fn new(test_runner: TR) -> Self {
        ApplicationServiceImpl { test_runner }
    }
}

impl<TR: TestRunner> ApplicationService for ApplicationServiceImpl<TR> {
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError> {
        let test_result: TestResult = self.test_runner.run_test(test_name)?.into();
        Ok(test_result)
    }
}
