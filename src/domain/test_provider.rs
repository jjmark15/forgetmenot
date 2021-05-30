use crate::domain::test::Test;

pub(crate) trait TestProvider {
    fn get(&self, test_name: &str) -> Result<&Test, GetTestError>;
}

pub(crate) struct TestProviderImpl {
    tests: Vec<Test>,
}

impl TestProviderImpl {
    pub(crate) fn new(tests: Vec<Test>) -> Self {
        TestProviderImpl { tests }
    }
}

impl TestProvider for TestProviderImpl {
    fn get(&self, test_name: &str) -> Result<&Test, GetTestError> {
        let tests: Vec<&Test> = self
            .tests
            .iter()
            .filter(|test| test.name() == test_name)
            .collect();

        match tests.first() {
            None => Err(TestNotFoundError(test_name.to_string()).into()),
            Some(test) => Ok(test),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetTestError {
    #[error(transparent)]
    TestNotFound(#[from] TestNotFoundError),
}

#[derive(Debug, thiserror::Error)]
#[error("test '{0}' not found")]
pub(crate) struct TestNotFoundError(pub(crate) String);
