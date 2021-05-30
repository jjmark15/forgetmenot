use crate::domain::command::TestCommand;

pub(crate) struct Test {
    name: String,
    command: TestCommand,
}

impl Test {
    pub(crate) fn new(name: String, command: TestCommand) -> Self {
        Test { name, command }
    }

    pub(crate) fn command(&self) -> &TestCommand {
        &self.command
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }
}
