pub(crate) struct TestResult {
    output: String,
}

impl TestResult {
    pub(crate) fn output(&self) -> &String {
        &self.output
    }
}

impl From<crate::domain::TestResult> for TestResult {
    fn from(domain_result: crate::domain::TestResult) -> Self {
        TestResult {
            output: domain_result.output().clone(),
        }
    }
}
