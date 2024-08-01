use app::App;
use clap::Parser;
use cli::Cli;
use executor::Executor;

mod app;
mod cli;
mod commands;
mod executor;
mod sqlite;
mod storage;
mod task;
mod task_printer;
mod task_repository;
mod task_service;

fn main() {
    let cli = Cli::parse();
    let app = App::new();

    Executor::run(&app, &cli);
}
