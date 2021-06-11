use std::path::PathBuf;

use structopt::StructOpt;

use crate::ports::cli::structopt::cli_options::ConfigCommand;

#[derive(StructOpt, Debug)]
pub(crate) struct RunTestCommand {
    /// Test command name
    pub(crate) test_name: String,

    /// Set config file path
    #[structopt(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

impl ConfigCommand for RunTestCommand {
    fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }
}
