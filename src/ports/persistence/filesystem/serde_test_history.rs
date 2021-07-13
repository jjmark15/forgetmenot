use std::collections::HashMap;

use crate::domain::{TestHistory, TestResult, VcsVersion};
use crate::ports::persistence::filesystem::serde_test_result::SerdeTestResult;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct SerdeTestHistory {
    inner: HashMap<String, SerdeTestResult>,
}

impl From<SerdeTestHistory> for TestHistory {
    fn from(history: SerdeTestHistory) -> Self {
        TestHistory::new(
            history
                .inner
                .into_iter()
                .map(|(version_string, test_result)| {
                    (
                        VcsVersion::new(version_string),
                        TestResult::from(test_result),
                    )
                })
                .collect(),
        )
    }
}

impl From<TestHistory> for SerdeTestHistory {
    fn from(test_history: TestHistory) -> Self {
        SerdeTestHistory {
            inner: test_history
                .results()
                .iter()
                .map(|(version, result)| (version.value().clone(), (*result).into()))
                .collect(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct SerdeTestHistories {
    test_histories: HashMap<String, SerdeTestHistory>,
}

impl SerdeTestHistories {
    pub(crate) fn get(&self, test_name: impl AsRef<str>) -> Option<&SerdeTestHistory> {
        self.test_histories.get(test_name.as_ref())
    }

    pub(crate) fn insert(&mut self, test_name: String, test_history: SerdeTestHistory) {
        self.test_histories.insert(test_name, test_history);
    }
}
