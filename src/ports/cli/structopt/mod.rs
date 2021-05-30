use std::error::Error;
use std::process::exit;
use std::rc::Rc;

use owo_colors::OwoColorize;
use structopt::StructOpt;

use crate::application::{ApplicationService, ApplicationServiceImpl};
use crate::domain::{Test, TestProviderImpl, TestRunnerImpl};
use crate::ports::cli::structopt::cli_options::CliOptions;
use crate::ports::command_execution::system_process::SystemProcessCommandExecutorAdapter;
use crate::ports::config::file::FileConfigReader;
use crate::ports::config::{ApplicationConfig, ConfigReader};

mod cli_options;

pub(crate) fn run_cli() {
    let opts: CliOptions = CliOptions::from_args();
    let config_reader = Rc::new(FileConfigReader::new());
    match opts {
        CliOptions::Run(run_command) => {
            let application_config = unwrap_or_exit_app_with_error_message(
                config_reader.read(run_command.config_path.as_path()),
            );
            let application_service = application_service(&application_config);
            let test_result = unwrap_or_exit_app_with_error_message(
                application_service.run_test(run_command.test_name.as_str()),
            );
            print!("{}", test_result.output());
        }
    }
}

fn application_service(config: &ApplicationConfig) -> impl ApplicationService {
    let command_executor = SystemProcessCommandExecutorAdapter::new();
    let test_provider =
        TestProviderImpl::new(config.tests().iter().cloned().map(Test::from).collect());
    let test_runner = TestRunnerImpl::new(command_executor, test_provider);
    ApplicationServiceImpl::new(test_runner)
}

fn unwrap_or_exit_app_with_error_message<U, E: Error>(result: Result<U, E>) -> U {
    match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}: {}", "error".bright_red(), err);
            exit(1);
        }
    }
}