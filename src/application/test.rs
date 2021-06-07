use crate::domain::{Test, TestCommand};

#[derive(Clone)]
pub(crate) struct ApplicationTest {
    name: String,
    command: String,
    description: Option<String>,
}

impl ApplicationTest {
    pub(crate) fn new(name: String, command: String, description: Option<String>) -> Self {
        ApplicationTest {
            name,
            command,
            description,
        }
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn description(&self) -> &Option<String> {
        &self.description
    }
}

impl From<ApplicationTest> for Test {
    fn from(test: ApplicationTest) -> Self {
        Test::new(test.name, TestCommand::new(test.command), test.description)
    }
}

impl From<&Test> for ApplicationTest {
    fn from(test: &Test) -> Self {
        ApplicationTest::new(
            test.name().clone(),
            test.command().string().clone(),
            test.description().clone(),
        )
    }
}
