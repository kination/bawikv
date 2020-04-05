use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::path::Path;

pub struct BawiKv {
    pub file_path: String,
}

impl BawiKv {
    pub fn put(&self, key: &str, value: &str) {
        let mut storage_file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(Path::new(&self.file_path))
        {
            Err(err) => panic!("Unknown error: {:?}", err),
            Ok(f) => f,
        };

        let mut bawi_map = HashMap::new();
        bawi_map.insert(key, value);

        write!(storage_file, "{}::{}\n", key, value);
    }

    pub fn get(&self, key: &str) -> Result<String, Error> {
        let storage_file = match OpenOptions::new()
            .read(true)
            .append(true)
            .open(Path::new(&self.file_path))
        {
            Err(err) => panic!("Unknown error: {:?}", err),
            Ok(f) => f,
        };

        let reader = BufReader::new(storage_file);

        for line in reader.lines() {
            let txt = match line {
                Err(err) => panic!("Unknown error: {:?}", err),
                Ok(line) => line,
            };

            let splited: Vec<&str> = txt.split("::").collect();
            if splited.len() == 2 {
                if splited[0] == key {
                    return Ok(String::from(splited[1]));
                }
            }
        }

        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid data: {}", key),
        ));
    }

    fn search() {}
}