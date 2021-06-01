pub(crate) struct TestResult {
    exit_code: i32,
}

impl TestResult {
    pub(crate) fn new(exit_code: i32) -> Self {
        TestResult { exit_code }
    }

    pub(crate) fn exit_code(&self) -> i32 {
        self.exit_code
    }
}
