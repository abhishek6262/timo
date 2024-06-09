use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

use dirs::data_local_dir;

pub struct Storage {
    storage_path: PathBuf,
}

fn get_storage_path() -> PathBuf {
    let mut storage_path = data_local_dir().unwrap();
    storage_path.push("timodata.log");
    storage_path
}

impl Storage {
    pub fn new() -> Self {
        Self {
            storage_path: get_storage_path(),
        }
    }

    pub fn clear(&self) {
        let file = File::options()
            .create(true)
            .write(true)
            .open(&self.storage_path)
            .unwrap();

        file.set_len(0).unwrap();
    }

    pub fn write(&self, text: &str) {
        let text = text.to_owned() + "\n";
        let file = File::options()
            .create(true)
            .append(true)
            .open(&self.storage_path)
            .unwrap();
        let mut writer = BufWriter::new(file);

        writer.write_all(text.as_bytes()).unwrap();
    }

    pub fn read(&self) -> BufReader<File> {
        let file = File::options()
            .create(true)
            .read(true)
            .open(&self.storage_path)
            .unwrap();

        BufReader::new(file)
    }
}
