use std::path::PathBuf;

use structopt::StructOpt;

use crate::ports::cli::structopt::cli_options::ConfigCommand;

#[derive(StructOpt, Debug)]
pub(crate) struct ListTestsCommand {
    /// Set config file path
    #[structopt(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

impl ConfigCommand for ListTestsCommand {
    fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }
}
