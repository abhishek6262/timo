use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Adds a new thought in the list")]
    Add {
        #[arg()]
        text: Vec<String>,
    },

    #[command(about = "Remove the thought from the list")]
    Remove {
        #[arg()]
        index: usize,
    },

    #[command(about = "Search thoughts in the list")]
    Search {
        #[arg()]
        key: String,
    },

    #[command(about = "Print all the thoughts in the list")]
    List,
}
