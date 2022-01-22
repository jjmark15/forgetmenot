use std::path::PathBuf;

use clap::Parser;
use derive_getters::Getters;

/// Project tests checklist and runner
#[derive(Parser, Debug)]
#[clap(name = "forgetmenot", version)]
pub(crate) enum CliOptions {
    /// Run a configured test
    Run(RunTestCommand),
    /// List all configured tests
    List(ListTestsCommand),
    /// Describe a configured test
    Describe(DescribeTestCommand),
}

#[derive(Parser, Debug, Getters)]
pub(crate) struct RunTestCommand {
    /// Test command name
    pub(crate) test_name: String,

    /// Set config file path
    #[clap(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

#[derive(Parser, Debug, Getters)]
pub(crate) struct ListTestsCommand {
    /// Set config file path
    #[clap(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

#[derive(Parser, Debug, Getters)]
pub(crate) struct DescribeTestCommand {
    /// Test command name
    pub(crate) test_name: String,

    /// Set config file path
    #[clap(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}
