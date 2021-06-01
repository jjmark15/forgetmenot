use std::path::PathBuf;
use structopt::StructOpt;

/// Project tests checklist and runner
#[derive(StructOpt, Debug)]
#[structopt(name = "forgetmenot")]
pub(crate) enum CliOptions {
    /// Run a configured test command
    Run(RunCommand),
}

#[derive(StructOpt, Debug)]
pub(crate) struct RunCommand {
    /// Test command name
    pub(crate) test_name: String,

    /// Set config file path
    #[structopt(short, long)]
    pub(crate) config_path: PathBuf,
}
