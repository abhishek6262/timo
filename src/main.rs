mod storage;

use std::io::BufRead;
use clap::{arg, Command};
use crate::storage::Storage;

fn main() {
    let matches = Command::new("Timo")
        .about(
            "Capture thoughts, ideas & experiences. \
            Fuzzy search lets you find entries instantly as you type. \
            Add, search & delete entries - all in your terminal.",
        )
        .subcommand_required(true)
        .subcommand(Command::new("add").about("Add a new item into the bucket").arg(arg!(<TEXT>)))
        .subcommand(Command::new("list").about("List items from the bucket"))
        .get_matches();

    let storage = Storage::new();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            if let Some(text) = sub_matches.get_one::<String>("TEXT") {
                let text = text.to_owned() + "\n";
                storage.wriite(&text);
            }
        },

        Some(("list", _)) => {
            let mut index = 0;

            for line in storage.read().lines() {
                let item = line.unwrap();
                index = index + 1;

                println!("[{index}]: {item}");
            }
        },
        _ => {
            //
        }
    }
}
