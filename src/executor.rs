use crate::task_service::TaskService;
use crate::{app::App, cli::Cli, commands::Commands};

pub struct Executor;

impl Executor {
    pub fn run(app: &App, cli: &Cli) {
        let task_service = TaskService::new(app);

        match &cli.command {
            Commands::Add { text, label } => task_service.add_task(text, label),
            Commands::Clear => task_service.clear_tasks(),
            Commands::Remove { indexes } => task_service.remove_task(indexes),
            Commands::Search { key, label } => task_service.search_task(key, label),
            Commands::List { label } => task_service.list_tasks(label),
        }
    }
}
