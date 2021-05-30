pub(crate) struct TestResult {
    output: String,
}

impl TestResult {
    pub(crate) fn new(output: String) -> Self {
        TestResult { output }
    }

    pub(crate) fn output(&self) -> &String {
        &self.output
    }
}
