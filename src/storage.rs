use dirs::data_local_dir;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

fn get_storage_path() -> PathBuf {
    let mut storage_path = data_local_dir().unwrap();

    storage_path.push("timo");
    storage_path
}

pub struct Storage {
    storage: File,
}

impl Storage {
    pub fn new() -> Self {
        let storage = File::options()
            .read(true)
            .append(true)
            .create(true)
            .open(get_storage_path())
            .unwrap();

        Self { storage }
    }

    pub fn clear(&self) {
        fs::write(get_storage_path(), "").unwrap();
    }

    pub fn write(&self, item: &str) {
        let mut buf_writer = BufWriter::new(&self.storage);

        buf_writer.write(item.as_bytes()).unwrap();
    }

    pub fn read(&self) -> BufReader<&File> {
        BufReader::new(&self.storage)
    }
}
