use crate::{
    app::App,
    cli::Cli,
    commands::Commands,
};
use crate::task_service::TaskService;

pub struct Executor;

impl Executor {
    pub fn run(app: &App, cli: &Cli) {
        let task_service = TaskService::new(app);

        match &cli.command {
            Commands::Add { text } => task_service.add_task(text),
            Commands::Clear => task_service.clear_tasks(),
            Commands::Remove { indexes } => task_service.remove_task(indexes),
            Commands::Search { key } => task_service.search_task(key),
            Commands::List => task_service.list_tasks(),
        }
    }
}
