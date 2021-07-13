use crate::domain::TestResult;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct SerdeTestResult {
    exit_code: i32,
}

impl From<SerdeTestResult> for TestResult {
    fn from(result: SerdeTestResult) -> Self {
        TestResult::new(result.exit_code)
    }
}

impl From<TestResult> for SerdeTestResult {
    fn from(result: TestResult) -> Self {
        SerdeTestResult {
            exit_code: result.exit_code(),
        }
    }
}
