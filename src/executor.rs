use crate::{
    app::App,
    cli::Cli,
    command_handlers::{add_item, list_items, remove_item, search_item},
    commands::Commands,
};

pub struct Executor;

impl Executor {
    pub fn run(app: &App, cli: &Cli) {
        match &cli.command {
            Commands::Add { text } => add_item(&app, text),
            Commands::Remove { index } => remove_item(&app, index),
            Commands::Search { key } => search_item(&app, key),
            Commands::List => list_items(&app),
        }
    }
}
