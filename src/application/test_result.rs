pub(crate) struct TestResult {
    output: String,
    exit_code: i32,
}

impl TestResult {
    pub(crate) fn output(&self) -> &String {
        &self.output
    }

    pub(crate) fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

impl From<crate::domain::TestResult> for TestResult {
    fn from(domain_result: crate::domain::TestResult) -> Self {
        TestResult {
            output: domain_result.output().clone(),
            exit_code: domain_result.exit_code(),
        }
    }
}
