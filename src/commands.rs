use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Add a new thought to the list")]
    Add {
        #[arg(required = true)]
        text: Vec<String>,

        #[arg(short, long)]
        label: Option<String>,
    },

    #[command(about = "Remove all thoughts from the list")]
    Clear {
        #[arg(required = true, long)]
        confirmed: bool,
    },

    #[command(about = "Remove a thought from the list")]
    Remove {
        #[arg(required = true)]
        ids: Vec<usize>,
    },

    #[command(about = "Search for thoughts in the list")]
    Search {
        #[arg(required = true)]
        key: Vec<String>,

        #[arg(short, long)]
        label: Option<String>,
    },

    #[command(about = "Print all thoughts in the list")]
    List {
        #[arg(short, long)]
        label: Option<String>,
    },
}
