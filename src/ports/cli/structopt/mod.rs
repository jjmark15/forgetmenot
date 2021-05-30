use structopt::StructOpt;

use crate::ports::cli::structopt::cli_options::CliOptions;

mod cli_options;

pub(crate) fn run_cli() {
    let _opts = CliOptions::from_args();
    println!("Hello, world!");
}
