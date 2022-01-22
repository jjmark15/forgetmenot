use crate::helpers::{Builder, DEFAULT_TEST_COMMAND, DEFAULT_TEST_DESCRIPTION, DEFAULT_TEST_NAME};

#[derive(Debug, serde::Serialize, derive_new::new)]
pub(crate) struct ApplicationConfig {
    tests: Vec<TestCommand>,
}

impl ApplicationConfig {
    pub(crate) fn single(test: TestCommand) -> Self {
        ApplicationConfig::new(vec![test])
    }
}

#[derive(Debug, serde::Serialize, derive_new::new)]
pub(crate) struct TestCommand {
    name: String,
    command: String,
    description: Option<String>,
}

impl TestCommand {
    pub(crate) fn builder() -> TestCommandBuilder {
        TestCommandBuilder::default()
    }
}

impl Default for TestCommand {
    fn default() -> Self {
        TestCommand::builder().build()
    }
}

pub(crate) struct TestCommandBuilder {
    name: String,
    command: String,
    description: Option<String>,
}

impl TestCommandBuilder {
    pub(crate) fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub(crate) fn with_command(mut self, command: String) -> Self {
        self.command = command;
        self
    }

    pub(crate) fn with_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
}

impl Builder for TestCommandBuilder {
    type Target = TestCommand;

    fn build(self) -> Self::Target {
        TestCommand::new(self.name, self.command, self.description)
    }
}

impl Default for TestCommandBuilder {
    fn default() -> Self {
        TestCommandBuilder {
            name: DEFAULT_TEST_NAME.to_string(),
            command: DEFAULT_TEST_COMMAND.to_string(),
            description: Some(DEFAULT_TEST_DESCRIPTION.to_string()),
        }
    }
}
