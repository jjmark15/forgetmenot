#[derive(Debug, serde::Serialize)]
pub(crate) struct ApplicationConfig {
    tests: Vec<TestCommand>,
}

impl ApplicationConfig {
    pub(crate) fn new(tests: Vec<TestCommand>) -> Self {
        ApplicationConfig { tests }
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct TestCommand {
    name: String,
    command: String,
    description: Option<String>,
}

impl TestCommand {
    pub(crate) fn new(name: String, command: String, description: Option<String>) -> Self {
        TestCommand {
            name,
            command,
            description,
        }
    }
}
