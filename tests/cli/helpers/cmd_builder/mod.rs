use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub(crate) use describe_test::DescribeTestCommandBuilder;
pub(crate) use list_tests::ListTestsCommandBuilder;
pub(crate) use run_test::RunTestCommandBuilder;
pub(crate) use subcommand::SubcommandBuilder;

use crate::helpers::cmd_builder::subcommand::SubcommandBase;
use crate::helpers::APPLICATION_NAME;

mod describe_test;
mod list_tests;
mod run_test;
mod subcommand;

pub(crate) struct CliCommandBuilder {}

impl CliCommandBuilder {
    pub(crate) fn run_test(test_name: &str) -> RunTestCommandBuilder {
        RunTestCommandBuilder::new(Self::subcommand_base(), test_name.to_string())
    }

    pub(crate) fn list_tests() -> ListTestsCommandBuilder {
        ListTestsCommandBuilder::new(Self::subcommand_base())
    }

    pub(crate) fn describe_test(test_name: &str) -> DescribeTestCommandBuilder {
        DescribeTestCommandBuilder::new(Self::subcommand_base(), test_name.to_string())
    }

    pub(crate) fn version() -> Assert {
        Self::binary_command().arg("-V").assert()
    }

    fn subcommand_base() -> SubcommandBase {
        SubcommandBase::new(Self::binary_command())
    }

    fn binary_command() -> Command {
        Command::cargo_bin(APPLICATION_NAME).unwrap()
    }
}
