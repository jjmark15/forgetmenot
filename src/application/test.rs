use crate::domain::{Test, TestCommand};

#[derive(Clone)]
pub(crate) struct ApplicationTest {
    name: String,
    command: String,
}

impl ApplicationTest {
    pub(crate) fn new(name: String, command: String) -> Self {
        ApplicationTest { name, command }
    }
}

impl From<ApplicationTest> for Test {
    fn from(test: ApplicationTest) -> Self {
        Test::new(test.name, TestCommand::new(test.command))
    }
}
