use std::io::BufRead;

use crate::app::App;

pub fn add_item(app: &App, text: &Vec<String>) {
    app.storage.write(&text.join(" "));
}

pub fn clear_items(app: &App) {
    app.storage.clear();
}

pub fn remove_item(app: &App, indexes: &Vec<usize>) {
    let texts: Vec<String> = app
        .storage
        .read()
        .lines()
        .enumerate()
        .filter(|predicate| !indexes.contains(&predicate.0))
        .map(|predicate| predicate.1.unwrap())
        .collect();

    app.storage.clear();

    if texts.len() > 0 {
        app.storage.write(&texts.join("\n"));
    }
}

pub fn search_item(app: &App, key: &Vec<String>) {
    let key = key.join(" ").to_lowercase();
    let mut texts = vec![];

    for line in app.storage.read().lines() {
        let text = line.unwrap();
        let search_text = text.to_lowercase();

        if search_text.contains(&key) {
            texts.push(text);
        }
    }

    for (index, line) in texts.iter().enumerate() {
        println!("[{index}]: {line}");
    }
}

pub fn list_items(app: &App) {
    for (index, line) in app.storage.read().lines().enumerate() {
        println!("[{index}]: {}", line.unwrap());
    }
}
