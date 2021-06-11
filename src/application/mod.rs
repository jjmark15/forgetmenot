use std::collections::HashMap;
use std::rc::Rc;

pub(crate) use test::*;

use crate::application::error::{DescribeTestError, GetChecklistError, RunTestError};
use crate::application::test_result::TestResult;
use crate::domain::{ChecklistService, TestProvider, TestRunner};

mod error;
mod test;
mod test_result;

pub(crate) trait ApplicationService {
    fn run_test(&self, test_name: &str) -> Result<TestResult, RunTestError>;

    fn list_tests(&self) -> Vec<ApplicationTest>;

    fn describe_test(&self, test_name: &str) -> Result<ApplicationTest, DescribeTestError>;

    fn get_test_checklist(&self) -> Result<HashMap<String, bool>, GetChecklistError>;
}

pub(crate) struct ApplicationServiceImpl<TR: TestRunner, TP: TestProvider, CS: ChecklistService> {
    test_runner: TR,
    test_provider: Rc<TP>,
    checklist_service: CS,
}

impl<TR, TP, CS> ApplicationServiceImpl<TR, TP, CS>
where
    TR: TestRunner,
    TP: TestProvider,
    CS: ChecklistService,
{
    pub(crate) fn new(test_runner: TR, test_provider: Rc<TP>, checklist_service: CS) -> Self {
        ApplicationServiceImpl {
            test_runner,
            test_provider,
            checklist_service,
        }
    }
}

impl<TR, TP, CS> ApplicationService for ApplicationServiceImpl<TR, TP, CS>
where
    TR: TestRunner,
    TP: TestProvider,
    CS: ChecklistService,
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

    fn describe_test(&self, test_name: &str) -> Result<ApplicationTest, DescribeTestError> {
        let test = self.test_provider.get(test_name)?;
        Ok(test.into())
    }

    fn get_test_checklist(&self) -> Result<HashMap<String, bool>, GetChecklistError> {
        self.checklist_service
            .get(self.test_provider.get_all())
            .and_then(|checklist| Ok(checklist.entries().clone()))
            .map_err(GetChecklistError::from)
    }
}
