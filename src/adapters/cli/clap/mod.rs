use std::error::Error;
use std::io;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use itertools::Itertools;
use owo_colors::OwoColorize;
use skim::prelude::{SkimItemReader, SkimOptionsBuilder};
use skim::Skim;

use crate::adapters::cli::clap::cli_options::{CliOptions, HasConfigPath, HasTestName};
use crate::adapters::command_execution::system_process::SystemProcessCommandExecutorAdapter;
use crate::adapters::config::file::{ConfigFileLocator, FileConfigReader};
use crate::adapters::config::{ApplicationConfig, ConfigReader};
use crate::application::{ApplicationService, ApplicationServiceImpl, ApplicationTest};
use crate::domain::{Test, TestProvider, TestProviderImpl, TestRunnerImpl};

mod cli_options;

pub(crate) fn run_cli() {
    let opts: CliOptions = CliOptions::parse();

    match opts {
        CliOptions::Run(command) => {
            let config_file_path = application_config_path(&command);
            let config_file_path_argument = &config_file_path;
            std::env::set_current_dir(config_file_path_argument.parent().unwrap()).unwrap();

            let application_service = application_service(application_config(&config_file_path));
            let test_name = get_test_name(&command, application_service.list_tests());

            if test_name.is_none() {
                exit(0);
            }

            print_discovered_config_parent_directory(config_file_path_argument);

            let test_result = unwrap_or_exit_app_with_error_message(
                application_service.run_test(&test_name.unwrap()),
            );
            exit(test_result.exit_code())
        }
        CliOptions::List(command) => {
            let config_file_path = application_config_path(&command);
            let config_file_path_argument = &config_file_path;
            std::env::set_current_dir(config_file_path_argument.parent().unwrap()).unwrap();
            print_discovered_config_parent_directory(config_file_path_argument);

            let application_service = application_service(application_config(&config_file_path));
            print_list_of_tests(application_service.list_tests());
        }
        CliOptions::Describe(command) => {
            let config_file_path = application_config_path(&command);
            let config_file_path_argument = &config_file_path;
            std::env::set_current_dir(config_file_path_argument.parent().unwrap()).unwrap();

            let application_service = application_service(application_config(&config_file_path));
            let test_name = get_test_name(&command, application_service.list_tests());

            if test_name.is_none() {
                exit(0);
            }

            print_discovered_config_parent_directory(config_file_path_argument);

            let application_test = unwrap_or_exit_app_with_error_message(
                application_service.describe_test(&test_name.unwrap()),
            );
            print_test_description(application_test);
        }
        CliOptions::Completions(command) => {
            generate(
                command.shell,
                &mut CliOptions::command(),
                env!("CARGO_PKG_NAME"),
                &mut io::stdout(),
            );
        }
    }
}

fn get_test_name(command: &impl HasTestName, test_names: Vec<ApplicationTest>) -> Option<String> {
    if let Some(name) = command.test_name() {
        return Some(name.to_string());
    }
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(
        test_names
            .into_iter()
            .map(|test| test.name().to_owned())
            .sorted()
            .join("\n"),
    ));

    let selected = Skim::run_with(&options, Some(items)).and_then(|out| {
        if out.is_abort {
            None
        } else {
            out.selected_items.first().cloned()
        }
    });

    selected.map(|s| s.text().to_string())
}

fn application_service(config: ApplicationConfig) -> impl ApplicationService {
    let command_executor = SystemProcessCommandExecutorAdapter::new();
    let mut test_provider = TestProviderImpl::new();
    unwrap_or_exit_app_with_error_message(
        test_provider.add_tests(config.tests().iter().cloned().map(Test::from).collect()),
    );
    let test_provider_ref = Rc::new(test_provider);
    let test_runner = TestRunnerImpl::new(command_executor, test_provider_ref.clone());
    ApplicationServiceImpl::new(test_runner, test_provider_ref)
}

fn application_config_path<C: HasConfigPath>(options: &C) -> PathBuf {
    options
        .config_path()
        .map(|p| std::fs::canonicalize(&p).unwrap_or_else(|_| p.clone()))
        .unwrap_or_else(|| {
            unwrap_or_exit_app_with_error_message(
                ConfigFileLocator::new().locate(
                    &std::env::current_dir().expect("could not determine current directory"),
                ),
            )
        })
}

fn application_config(config_path: &Path) -> ApplicationConfig {
    let config_reader = FileConfigReader::new();
    unwrap_or_exit_app_with_error_message(config_reader.read(config_path))
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
