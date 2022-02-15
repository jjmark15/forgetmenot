use crate::adapters::cli::clap::run_cli;

mod adapters;
mod application;
mod domain;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn run(&self) {
        run_cli();
    }
}
