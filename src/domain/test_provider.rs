use std::collections::HashMap;

use crate::domain::test::Test;

pub(crate) trait TestProvider {
    fn add_tests(&mut self, tests: Vec<Test>) -> Result<(), TestAlreadyExists>;

    fn get(&self, test_name: &str) -> Result<&Test, GetTestError>;

    fn get_all(&self) -> Vec<&Test>;
}

#[derive(derive_new::new)]
pub(crate) struct TestProviderImpl {
    #[new(default)]
    tests: HashMap<String, Test>,
}

impl TestProvider for TestProviderImpl {
    fn add_tests(&mut self, tests: Vec<Test>) -> Result<(), TestAlreadyExists> {
        for test in tests.into_iter() {
            let test_name = test.name().clone();
            if self.tests.contains_key(test.name()) {
                return Err(TestAlreadyExists(test_name));
            }
            self.tests.insert(test_name, test);
        }

        Ok(())
    }

    fn get(&self, test_name: &str) -> Result<&Test, GetTestError> {
        match self.tests.get(test_name) {
            None => Err(TestNotFoundError(test_name.to_string()).into()),
            Some(test) => Ok(test),
        }
    }

    fn get_all(&self) -> Vec<&Test> {
        self.tests.values().collect()
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

#[derive(Debug, thiserror::Error)]
#[error("test with name '{0}' already exists")]
pub(crate) struct TestAlreadyExists(pub(crate) String);
