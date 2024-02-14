mod storage_directory;

use std::{
    fs,
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};
pub use storage_directory::StorageDirectory;

pub fn get_file_path(
    storage_directory: StorageDirectory,
    application_name: &str,
    data_name: &str,
) -> PathBuf {
    let mut path = storage_directory.get_path();
    path.push(application_name);
    path.push(data_name);
    path.set_extension("json");

    path
}

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
        let file = fs::File::open(&self.file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &data).unwrap(); // todo: remove unwrap

        Ok(())
    }

    pub fn retrieve<'a, T: for<'de> serde::Deserialize<'de>>(&self) -> Result<T, io::Error> {
        let file = fs::File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader).unwrap(); // todo: remove unwrap
        Ok(data)
    }
}
