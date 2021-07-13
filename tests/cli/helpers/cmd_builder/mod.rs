use std::path::Path;

use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub(crate) use describe_test::DescribeTestCommandBuilder;
pub(crate) use list_tests::ListTestsCommandBuilder;
pub(crate) use run_test::RunTestCommandBuilder;
pub(crate) use subcommand::SubcommandBuilder;

use crate::helpers::cmd_builder::subcommand::SubcommandBase;
use crate::helpers::cmd_builder::view_checklist::ViewChecklistCommandBuilder;
use crate::helpers::APPLICATION_NAME;

mod describe_test;
mod list_tests;
mod run_test;
mod subcommand;
mod view_checklist;

pub(crate) struct CliCommandBuilder {
    binary_command: Command,
}

impl CliCommandBuilder {
    pub(crate) fn new(home_directory: impl AsRef<Path>) -> Self {
        CliCommandBuilder {
            binary_command: Self::binary_command(home_directory),
        }
    }

    pub(crate) fn run_test(self, test_name: &str) -> RunTestCommandBuilder {
        RunTestCommandBuilder::new(self.subcommand_base(), test_name.to_string())
    }

    pub(crate) fn list_tests(self) -> ListTestsCommandBuilder {
        ListTestsCommandBuilder::new(self.subcommand_base())
    }

    pub(crate) fn describe_test(self, test_name: &str) -> DescribeTestCommandBuilder {
        DescribeTestCommandBuilder::new(self.subcommand_base(), test_name.to_string())
    }

    pub(crate) fn view_checklist(self) -> ViewChecklistCommandBuilder {
        ViewChecklistCommandBuilder::new(self.subcommand_base())
    }

    pub(crate) fn version(mut self) -> Assert {
        self.binary_command.arg("-V").assert()
    }

    fn subcommand_base(self) -> SubcommandBase {
        SubcommandBase::new(self.binary_command)
    }

    fn binary_command(_home_directory: impl AsRef<Path>) -> Command {
        Command::cargo_bin(APPLICATION_NAME).unwrap()
    }
}
