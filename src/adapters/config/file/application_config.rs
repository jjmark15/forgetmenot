use crate::adapters::config::ApplicationConfig;
use crate::application::ApplicationTest;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SerdeApplicationConfig {
    tests: Vec<SerdeTest>,
}

impl From<SerdeApplicationConfig> for ApplicationConfig {
    fn from(config: SerdeApplicationConfig) -> Self {
        ApplicationConfig::new(
            config
                .tests
                .into_iter()
                .map(ApplicationTest::from)
                .collect(),
        )
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SerdeTest {
    name: String,
    command: String,
    description: Option<String>,
}

impl From<SerdeTest> for ApplicationTest {
    fn from(test: SerdeTest) -> Self {
        ApplicationTest::new(test.name, test.command, test.description)
    }
}
