use crate::domain::{Test, TestCommand};

#[derive(Clone, derive_new::new)]
pub(crate) struct ApplicationTest {
    name: String,
    command: String,
    description: Option<String>,
}

impl ApplicationTest {
    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn description(&self) -> &Option<String> {
        &self.description
    }

    pub(crate) fn command(&self) -> &String {
        &self.command
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
