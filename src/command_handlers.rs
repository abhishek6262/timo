use std::io::BufRead;

use crate::app::App;

pub fn add_item(app: &App, text: &Vec<String>) {
    app.storage.write(&text.join(" "));
}

pub fn remove_item(app: &App, index: &usize) {
    println!("{:?}", index);
}

pub fn search_item(app: &App, key: &String) {
    let key = key.to_lowercase();
    let mut texts = vec![];

    for line in app.storage.read().lines() {
        let text = line.unwrap().to_lowercase();

        if text.contains(&key) {
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
