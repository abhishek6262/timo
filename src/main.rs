mod storage;

use crate::storage::Storage;
use clap::{arg, Command};
use std::io::BufRead;

fn main() {
    let matches = Command::new("Timo")
        .about(
            "Capture thoughts, ideas & experiences. \
            Fuzzy search lets you find entries instantly as you type. \
            Add, search & delete entries - all in your terminal.",
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Add a new item into the list")
                .arg(arg!(<TEXT>)),
        )
        .subcommand(
            Command::new("remove")
                .about("Deletes an item from the list")
                .arg(arg!(<INDEX>)),
        )
        .subcommand(
            Command::new("search")
                .about("Search a keyword in the list")
                .arg(arg!(<KEY>)),
        )
        .subcommand(Command::new("list").about("List items in the bucket"))
        .get_matches();

    let storage = Storage::new();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            if let Some(text) = sub_matches.get_one::<String>("TEXT") {
                let text = text.to_owned() + "\n";
                storage.write(&text);
            }
        }

        Some(("remove", sub_matches)) => {
            if let Some(index) = sub_matches.get_one::<String>("INDEX") {
                let index: usize = index.parse().unwrap();
                let mut items = String::new();

                let mut i = 0;

                for item in storage.read().lines() {
                    i += 1;

                    if index == i {
                        continue;
                    }

                    let item = format!("{}\n", item.unwrap());
                    items.push_str(&item);
                }

                storage.clear();
                storage.write(&items);
            }
        }

        Some(("search", sub_matches)) => {
            if let Some(key) = sub_matches.get_one::<String>("KEY") {
                let mut items = vec![];

                for item in storage.read().lines() {
                    let item = item.unwrap();

                    if item.contains(key) {
                        items.push(item);
                    }
                }

                let mut index = 1;

                for item in items {
                    println!("[{index}]: {item}");
                    index += 1;
                }
            }
        }

        Some(("list", _)) => {
            let mut index = 0;

            for line in storage.read().lines() {
                let item = line.unwrap();
                index = index + 1;

                println!("[{index}]: {item}");
            }
        }

        _ => {
            //
        }
    }
}
