pub(crate) struct TestResult {
    output: String,
    exit_code: i32,
}

impl TestResult {
    pub(crate) fn new(output: String, exit_code: i32) -> Self {
        TestResult { output, exit_code }
    }

    pub(crate) fn output(&self) -> &String {
        &self.output
    }

    pub(crate) fn exit_code(&self) -> i32 {
        self.exit_code
    }
}
