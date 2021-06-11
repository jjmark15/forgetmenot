use std::path::PathBuf;

use structopt::StructOpt;

use crate::ports::cli::structopt::cli_options::ConfigCommand;

#[derive(StructOpt, Debug)]
pub(crate) struct ViewChecklistCommand {
    /// Set config file path
    #[structopt(short, long)]
    pub(crate) config_path: Option<PathBuf>,
}

impl ConfigCommand for ViewChecklistCommand {
    fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }
}
