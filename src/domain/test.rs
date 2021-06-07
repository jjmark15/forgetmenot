use crate::domain::command::TestCommand;

pub(crate) struct Test {
    name: String,
    command: TestCommand,
    description: Option<String>,
}

impl Test {
    pub(crate) fn new(name: String, command: TestCommand, description: Option<String>) -> Self {
        Test {
            name,
            command,
            description,
        }
    }

    pub(crate) fn command(&self) -> &TestCommand {
        &self.command
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn description(&self) -> &Option<String> {
        &self.description
    }
}
