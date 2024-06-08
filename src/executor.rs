use crate::{
    app::App,
    cli::Cli,
    command_handlers::{add_item, clear_items, list_items, remove_item, search_item},
    commands::Commands,
};

pub struct Executor;

impl Executor {
    pub fn run(app: &App, cli: &Cli) {
        match &cli.command {
            Commands::Add { text } => add_item(&app, text),
            Commands::Clear => clear_items(&app),
            Commands::Remove { indexes } => remove_item(&app, indexes),
            Commands::Search { key } => search_item(&app, key),
            Commands::List => list_items(&app),
        }
    }
}
