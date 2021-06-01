use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub(crate) struct ApplicationConfig {
    tests: HashMap<String, Test>,
}

impl ApplicationConfig {
    pub(crate) fn new(tests: HashMap<String, Test>) -> Self {
        ApplicationConfig { tests }
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Test {
    command: String,
}

impl Test {
    pub(crate) fn new(command: String) -> Self {
        Test { command }
    }
}
