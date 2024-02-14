mod persistent_data;
mod persistent_data_builder;
mod storage_directory;

pub use persistent_data::PersistentData;
pub use persistent_data_builder::PersistentDataBuilder;
pub use storage_directory::StorageDirectory;

use std::{fs, path::PathBuf};

fn get_application_path(storage_directory: StorageDirectory, application_name: &str) -> PathBuf {
    let mut path = storage_directory.get_path();
    path.push(application_name);

    path
}

fn get_file_path(
    storage_directory: StorageDirectory,
    application_name: &str,
    data_name: &str,
) -> PathBuf {
    let mut path = get_application_path(storage_directory, application_name);
    path.push(data_name);
    path.set_extension("json");

    path
}

pub fn delete_data_path(storage_directory: StorageDirectory, application_name: &str) {
    let application_path = get_application_path(storage_directory, application_name);

    // todo: should probably catch when it doesnt delete
    let _ = fs::remove_dir(application_path);
}
