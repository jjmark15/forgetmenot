use std::collections::HashMap;

use crate::application::ApplicationTest;
use crate::ports::config::ApplicationConfig;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SerdeApplicationConfig {
    tests: HashMap<String, SerdeTest>,
}

impl From<SerdeApplicationConfig> for ApplicationConfig {
    fn from(config: SerdeApplicationConfig) -> Self {
        ApplicationConfig::new(test_hashmap_to_vec(config.tests))
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SerdeTest {
    command: String,
}

fn test_hashmap_to_vec(tests: HashMap<String, SerdeTest>) -> Vec<ApplicationTest> {
    tests
        .into_iter()
        .map(|(name, serde_test)| ApplicationTest::new(name, serde_test.command))
        .collect()
}
