use crate::get_file_path;
use crate::StorageDirectory;
use std::{
    fs,
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

pub struct PersistentData {
    file_path: PathBuf,
}

impl PersistentData {
    pub fn new(
        storage_directory: StorageDirectory,
        application_name: &str,
        data_name: &str,
    ) -> Self {
        Self {
            file_path: get_file_path(storage_directory, application_name, data_name),
        }
    }

    pub fn store<T: serde::Serialize>(&self, data: &T) -> Result<(), io::Error> {
        // parent directories might not exist
        let parent_file_path = self.file_path.parent().unwrap();
        fs::create_dir_all(&parent_file_path)?;

        let file = fs::OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .open(&self.file_path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &data).unwrap(); // todo: remove unwrap
        Ok(())
    }

    pub fn retrieve<'a, T: for<'de> serde::Deserialize<'de>>(&self) -> Result<T, io::Error> {
        let file = fs::OpenOptions::new()
            .write(false)
            .read(true)
            .open(&self.file_path)?;
        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader).unwrap(); // todo: remove unwrap
        Ok(data)
    }
}
