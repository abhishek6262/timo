use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Add a new thought to the list")]
    Add {
        #[arg()]
        text: Vec<String>,

        #[arg(short = 'l', long = "label")]
        label: Option<String>,
    },

    #[command(about = "Remove all thoughts from the list")]
    Clear,

    #[command(about = "Remove a thought from the list")]
    Remove {
        #[arg()]
        indexes: Vec<usize>,
    },

    #[command(about = "Search for thoughts in the list")]
    Search {
        #[arg()]
        key: Vec<String>,

        #[arg(short = 'l', long = "label")]
        label: Option<String>,
    },

    #[command(about = "Print all thoughts in the list")]
    List {
        #[arg(short = 'l', long = "label")]
        label: Option<String>,
    },
}
