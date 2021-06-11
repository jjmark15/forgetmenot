use crate::domain::TestResult;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SerdeTestResult {
    exit_code: i32,
}

impl From<SerdeTestResult> for TestResult {
    fn from(result: SerdeTestResult) -> Self {
        TestResult::new(result.exit_code)
    }
}
