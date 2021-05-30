pub(crate) struct TestCommand {
    string: String,
}

impl TestCommand {
    pub(crate) fn new(string: String) -> Self {
        TestCommand { string }
    }

    pub(crate) fn string(&self) -> &String {
        &self.string
    }
}
