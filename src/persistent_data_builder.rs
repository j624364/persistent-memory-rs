use crate::PersistentData;
use crate::StorageDirectory;

pub struct PersistentDataBuilder<'a> {
    storage_directory: StorageDirectory,
    application_name: &'a str,
    data_name: &'a str,
}

impl<'a> PersistentDataBuilder<'a> {
    // data must be specified so might as well make it mandatory here
    pub fn new(application_name: &'a str, data_name: &'a str) -> Self {
        Self {
            storage_directory: StorageDirectory::default(),
            application_name,
            data_name,
        }
    }

    pub fn storage_directory(
        mut self,
        storage_directory: StorageDirectory,
    ) -> PersistentDataBuilder<'a> {
        self.storage_directory = storage_directory;
        self
    }

    pub fn build(self) -> PersistentData {
        PersistentData::new(
            self.storage_directory,
            self.application_name,
            self.data_name,
        )
    }
}
