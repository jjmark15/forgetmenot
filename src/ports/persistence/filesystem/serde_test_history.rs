use std::collections::HashMap;

use crate::domain::{TestHistory, TestResult, VcsVersion};
use crate::ports::persistence::filesystem::serde_test_result::SerdeTestResult;

#[derive(serde::Deserialize)]
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
