use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

use dirs::data_local_dir;

pub struct Storage {
    reader: File,
    writer: File,
}

fn get_storage_path() -> PathBuf {
    let mut storage_path = data_local_dir().unwrap();

    storage_path.push("timodata.log");
    storage_path
}

fn ensure_path_exists(path: &PathBuf) {
    if !path.exists() {
        fs::write(path, "").unwrap();
    }
}

impl Storage {
    pub fn new() -> Self {
        let storage_path = get_storage_path();

        ensure_path_exists(&storage_path);

        let reader_path = storage_path.clone();
        let writer_path = storage_path.clone();

        let reader = File::options().read(true).open(reader_path).unwrap();
        let writer = File::options().append(true).open(writer_path).unwrap();

        Self { reader, writer }
    }

    pub fn write(&self, text: &str) {
        let text = text.to_owned() + "\n";
        let mut writer = BufWriter::new(&self.writer);

        writer.write(&text.as_bytes()).unwrap();
    }

    pub fn read(&self) -> BufReader<&File> {
        BufReader::new(&self.reader)
    }
}
