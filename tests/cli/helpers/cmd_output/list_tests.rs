use std::fmt::{Display, Formatter};

use crate::helpers::cmd_output::{green, yellow};

pub(crate) struct ListTestsOutput {
    lines: Vec<ListTestsLine>,
}

impl ListTestsOutput {
    pub(crate) fn single(line: ListTestsLine) -> Self {
        ListTestsOutput { lines: vec![line] }
    }

    pub(crate) fn new(lines: impl AsRef<[ListTestsLine]>) -> Self {
        ListTestsOutput {
            lines: lines.as_ref().to_vec(),
        }
    }
}

impl Display for ListTestsOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .lines
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        writeln!(f, "{}", lines)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ListTestsLine {
    test_name: String,
    test_description: Option<String>,
}

impl ListTestsLine {
    pub(crate) fn new(test_name: impl AsRef<str>, test_description: impl AsRef<str>) -> Self {
        ListTestsLine {
            test_name: test_name.as_ref().to_string(),
            test_description: Some(test_description.as_ref().to_string()),
        }
    }

    pub(crate) fn without_description(test_name: impl AsRef<str>) -> Self {
        ListTestsLine {
            test_name: test_name.as_ref().to_string(),
            test_description: None,
        }
    }
}

impl Display for ListTestsLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let test_name = green(&self.test_name);

        match &self.test_description {
            None => write!(f, "{}", test_name),
            Some(test_description) => write!(f, "{} - {}", test_name, yellow(test_description)),
        }
    }
}
