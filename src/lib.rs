use crate::ports::cli::clap::run_cli;

mod application;
mod domain;
mod ports;

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
