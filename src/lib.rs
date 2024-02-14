mod storage_directory;

use std::{fs, io, path::PathBuf};
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

pub fn store<T: serde::Serialize>(
    storage_directory: StorageDirectory,
    application_name: &str,
    data_name: &str,
    data: &T,
) -> Result<(), io::Error> {
    let file_path = get_file_path(storage_directory, application_name, data_name);
    let serialised_json = serde_json::to_string(data).unwrap(); // todo: remove unwrap

    fs::write(file_path, serialised_json)
}
