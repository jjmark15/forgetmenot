use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;

use owo_colors::OwoColorize;
use structopt::StructOpt;

use crate::application::{ApplicationService, ApplicationServiceImpl, ApplicationTest};
use crate::domain::{
    Test, TestProvider, TestProviderImpl, TestRunnerImpl, VcsVersionChecklistService,
};
use crate::ports::cli::structopt::cli_options::{CliOptions, ConfigCommand};
use crate::ports::command_execution::system_process::SystemProcessCommandExecutorAdapter;
use crate::ports::config::file::{ConfigFileLocator, FileConfigReader};
use crate::ports::config::{ApplicationConfig, ConfigReader};
use crate::ports::persistence::filesystem::FilesystemTestHistoryRepositoryAdapter;
use crate::ports::version_control::git2::Git2VcsRepositoryProvidersAdapter;

mod cli_options;
mod describe_test;
mod list_tests_command;
mod run_test_command;
mod view_checklist_command;

pub(crate) fn run_cli() {
    let opts: CliOptions = CliOptions::from_args();
    match opts {
        CliOptions::Run(cli_command) => {
            let application_service = prepare_app_for_config_driven_command(&cli_command);
            let test_result = unwrap_or_exit_app_with_error_message(
                application_service.run_test(cli_command.test_name.as_str()),
            );
            exit(test_result.exit_code())
        }
        CliOptions::List(cli_command) => {
            let application_service = prepare_app_for_config_driven_command(&cli_command);
            print_list_of_tests(application_service.list_tests());
        }
        CliOptions::Describe(cli_command) => {
            let application_service = prepare_app_for_config_driven_command(&cli_command);
            let application_test = unwrap_or_exit_app_with_error_message(
                application_service.describe_test(cli_command.test_name.as_str()),
            );
            print_test_description(application_test);
        }
        CliOptions::Check(cli_command) => {
            let application_service = prepare_app_for_config_driven_command(&cli_command);
            let checklist =
                unwrap_or_exit_app_with_error_message(application_service.get_test_checklist());
            print_test_checklist(checklist);
        }
    }
}

fn prepare_app_for_config_driven_command(
    config_command: &impl ConfigCommand,
) -> impl ApplicationService {
    let application_config_path = application_config_path(config_command);
    let project_path = application_config_path.parent().unwrap();
    std::env::set_current_dir(&project_path).unwrap();
    let application_config = application_config(&application_config_path);
    print_discovered_config_parent_directory(application_config_path.as_path());
    application_service(&application_config, project_path)
}

fn application_service(config: &ApplicationConfig, project_path: &Path) -> impl ApplicationService {
    let command_executor = SystemProcessCommandExecutorAdapter::new();
    let mut test_provider = TestProviderImpl::new();
    unwrap_or_exit_app_with_error_message(
        test_provider.add_tests(config.tests().iter().cloned().map(Test::from).collect()),
    );
    let test_provider_ref = Rc::new(test_provider);
    let filesystem_test_history_repository_adapter =
        Rc::new(FilesystemTestHistoryRepositoryAdapter::new());
    let vcs_repository_provider = Rc::new(Git2VcsRepositoryProvidersAdapter::new());
    let test_runner = TestRunnerImpl::new(
        command_executor,
        test_provider_ref.clone(),
        filesystem_test_history_repository_adapter.clone(),
        vcs_repository_provider.clone(),
    );
    let checklist_service = VcsVersionChecklistService::new(
        vcs_repository_provider,
        project_path.to_path_buf(),
        filesystem_test_history_repository_adapter,
    );
    ApplicationServiceImpl::new(test_runner, test_provider_ref, checklist_service)
}

fn application_config_path(run_command: &impl ConfigCommand) -> PathBuf {
    let config_locator = ConfigFileLocator::new();
    run_command.config_path().clone().unwrap_or_else(|| {
        let current_directory =
            std::env::current_dir().expect("could not determine current directory");
        unwrap_or_exit_app_with_error_message(config_locator.locate(&current_directory))
    })
}

fn application_config(config_path: &Path) -> ApplicationConfig {
    let config_reader = FileConfigReader::new();
    unwrap_or_exit_app_with_error_message(config_reader.read(&config_path))
}

fn unwrap_or_exit_app_with_error_message<U, E: Error>(result: Result<U, E>) -> U {
    match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}: {}", "error".bright_red().bold(), err);
            exit(1);
        }
    }
}

fn print_list_of_tests(mut test_names: Vec<ApplicationTest>) {
    test_names.sort_by_key(|test| test.name().clone());
    let test_lines = test_names
        .iter()
        .map(list_test_line)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", test_lines);
}

fn print_test_checklist(test_checklist: HashMap<String, bool>) {
    let checklist_output: String = test_checklist
        .into_iter()
        .map(|(test_name, checked)| checklist_line(test_name, checked))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", checklist_output);
}

fn checklist_line(test_name: String, checked: bool) -> String {
    let checked_symbol = match checked {
        true => '✔',
        false => '❌',
    };
    format!("{}: {}", test_name.bright_yellow(), checked_symbol)
}

fn list_test_line(test: &ApplicationTest) -> String {
    let test_name = test.name().bright_green();
    match test.description() {
        None => test_name.to_string(),
        Some(description) => format!("{} - {}", test_name, description.bright_yellow()),
    }
}

fn print_discovered_config_parent_directory(config_path: &Path) {
    let message = format!(
        "Discovered {} config",
        fsio::path::get_basename(
            config_path
                .parent()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .as_str()
        )
        .unwrap()
        .bright_purple()
    );
    println!("{}\n", message);
}

fn print_test_description(test: ApplicationTest) {
    let mut lines = vec![test_description_line("name", test.name())];
    if let Some(description) = test.description() {
        lines.push(test_description_line("description", description));
    }
    lines.push(test_description_line("command", test.command()));

    println!("{}", lines.join("\n"))
}

fn test_description_line<S: AsRef<str>>(key: &str, value: S) -> String {
    format!("{}: {}", key.bright_yellow(), value.as_ref())
}
