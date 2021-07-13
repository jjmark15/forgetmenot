use std::collections::HashMap;

use crate::domain::{TestResult, VcsVersion};

pub(crate) trait TestHistoryRepository {
    fn get(&self, test_name: impl AsRef<str>) -> Result<TestHistory, GetTestHistoryError>;

    fn store(
        &self,
        test_name: impl AsRef<str>,
        test_history: TestHistory,
    ) -> Result<(), StoreTestHistoryError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetTestHistoryError {
    #[error("test history not found")]
    NotFound,
}

#[derive(Debug, thiserror::Error, Default)]
#[error("failed to store test history")]
pub(crate) struct StoreTestHistoryError;

#[derive(Default)]
pub(crate) struct TestHistory {
    inner: HashMap<VcsVersion, TestResult>,
}

impl TestHistory {
    pub(crate) fn new(inner: HashMap<VcsVersion, TestResult>) -> Self {
        TestHistory { inner }
    }

    pub(crate) fn result_for(&self, version: &VcsVersion) -> Option<&TestResult> {
        self.inner.get(version)
    }

    pub(crate) fn update_result_for(&mut self, version: VcsVersion, result: TestResult) {
        self.inner.insert(version, result);
    }

    pub(crate) fn results(&self) -> &HashMap<VcsVersion, TestResult> {
        &self.inner
    }
}
