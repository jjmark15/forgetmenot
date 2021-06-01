use std::error::Error;
use std::process::exit;

use owo_colors::OwoColorize;
use structopt::StructOpt;

use crate::application::{ApplicationService, ApplicationServiceImpl};
use crate::domain::{Test, TestProvider, TestProviderImpl, TestRunnerImpl};
use crate::ports::cli::structopt::cli_options::{CliOptions, RunCommand};
use crate::ports::command_execution::system_process::SystemProcessCommandExecutorAdapter;
use crate::ports::config::file::{ConfigFileLocator, FileConfigReader};
use crate::ports::config::{ApplicationConfig, ConfigReader};

mod cli_options;

pub(crate) fn run_cli() {
    let opts: CliOptions = CliOptions::from_args();
    match opts {
        CliOptions::Run(run_command) => {
            let application_config = application_config(&run_command);
            let application_service = application_service(&application_config);
            let test_result = unwrap_or_exit_app_with_error_message(
                application_service.run_test(run_command.test_name.as_str()),
            );
            exit(test_result.exit_code())
        }
    }
}

fn application_service(config: &ApplicationConfig) -> impl ApplicationService {
    let command_executor = SystemProcessCommandExecutorAdapter::new();
    let mut test_provider = TestProviderImpl::new();
    unwrap_or_exit_app_with_error_message(
        test_provider.add_tests(config.tests().iter().cloned().map(Test::from).collect()),
    );
    let test_runner = TestRunnerImpl::new(command_executor, test_provider);
    ApplicationServiceImpl::new(test_runner)
}

fn application_config(run_command: &RunCommand) -> ApplicationConfig {
    let config_reader = FileConfigReader::new();
    let config_locator = ConfigFileLocator::new();
    let config_path = run_command.config_path.clone().unwrap_or_else(|| {
        let current_directory =
            std::env::current_dir().expect("could not determine current directory");
        unwrap_or_exit_app_with_error_message(config_locator.locate(&current_directory))
    });

    unwrap_or_exit_app_with_error_message(config_reader.read(&config_path))
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
