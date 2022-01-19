use std::path::PathBuf;

use clap::Parser;

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

#[derive(Parser, Debug)]
pub(crate) struct RunTestCommand {
    /// Test command name
    pub(crate) test_name: String,

    /// Set config file path
    #[clap(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

impl ConfigCommand for RunTestCommand {
    fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }
}

#[derive(Parser, Debug)]
pub(crate) struct ListTestsCommand {
    /// Set config file path
    #[clap(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

impl ConfigCommand for ListTestsCommand {
    fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }
}

#[derive(Parser, Debug)]
pub(crate) struct DescribeTestCommand {
    /// Test command name
    pub(crate) test_name: String,

    /// Set config file path
    #[clap(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

impl ConfigCommand for DescribeTestCommand {
    fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }
}

pub(crate) trait ConfigCommand {
    fn config_path(&self) -> &Option<PathBuf>;
}
