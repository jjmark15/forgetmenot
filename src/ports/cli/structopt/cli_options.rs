use std::path::PathBuf;

use structopt::StructOpt;

use crate::ports::cli::structopt::describe_test::DescribeTestCommand;
use crate::ports::cli::structopt::list_tests_command::ListTestsCommand;
use crate::ports::cli::structopt::run_test_command::RunTestCommand;
use crate::ports::cli::structopt::view_checklist_command::ViewChecklistCommand;

/// Project tests checklist and runner
#[derive(StructOpt, Debug)]
#[structopt(name = "forgetmenot")]
pub(crate) enum CliOptions {
    /// Run a configured test
    Run(RunTestCommand),
    /// List all configured tests
    List(ListTestsCommand),
    /// Describe a configured test
    Describe(DescribeTestCommand),
    /// View test checklist
    Check(ViewChecklistCommand),
}

pub(crate) trait ConfigCommand {
    fn config_path(&self) -> &Option<PathBuf>;
}
