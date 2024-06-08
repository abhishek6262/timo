use app::App;
use clap::Parser;
use cli::Cli;
use executor::Executor;

mod app;
mod cli;
mod command_handlers;
mod commands;
mod executor;
mod storage;

fn main() {
    let cli = Cli::parse();
    let app = App::new();

    Executor::run(&app, &cli);
}
