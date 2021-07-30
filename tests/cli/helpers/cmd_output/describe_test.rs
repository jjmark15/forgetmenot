use std::fmt::{Display, Formatter};

use crate::helpers::cmd_output::yellow;

pub(crate) struct DescribeTestOutput {
    name: String,
    description: Option<String>,
    command: String,
}

impl DescribeTestOutput {
    pub(crate) fn new(
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        command: impl AsRef<str>,
    ) -> Self {
        DescribeTestOutput {
            name: name.as_ref().to_string(),
            description: Some(description.as_ref().to_string()),
            command: command.as_ref().to_string(),
        }
    }

    pub(crate) fn without_description(name: impl AsRef<str>, command: impl AsRef<str>) -> Self {
        DescribeTestOutput {
            name: name.as_ref().to_string(),
            description: None,
            command: command.as_ref().to_string(),
        }
    }
}

impl Display for DescribeTestOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let values: Vec<String> = vec![
            Some(key_value_string("name", &self.name)),
            self.description
                .as_ref()
                .map(|description| key_value_string("description", description)),
            Some(key_value_string("command", &self.command)),
        ]
        .into_iter()
        .flatten()
        .collect();

        writeln!(f, "{}", values.join("\n"))
    }
}

fn key_value_string(key: &str, value: &str) -> String {
    format!("{}: {}", yellow(key), value)
}
