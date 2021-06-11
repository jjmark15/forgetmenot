use std::collections::HashMap;

use crate::domain::{TestResult, VcsVersion};

pub(crate) trait TestHistoryRepository {
    fn get(&self, test_name: impl AsRef<str>) -> Result<TestHistory, GetTestHistoryError>;
}

#[derive(Debug, thiserror::Error, Default)]
#[error("failed to get test history")]
pub(crate) struct GetTestHistoryError;

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
}
