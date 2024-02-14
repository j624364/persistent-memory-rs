use std::path::PathBuf;

#[derive(Default)]
pub enum StorageDirectory {
    Home,
    Cache,
    Config,
    ConfigLocal,
    Data,
    #[default]
    DataLocal,
    Executable,
    Desktop,
}

impl StorageDirectory {
    pub fn get_location(&self) -> Option<PathBuf> {
        match self {
            Self::Home => dirs::home_dir(),
            Self::Cache => dirs::cache_dir(),
            Self::Config => dirs::config_dir(),
            Self::ConfigLocal => dirs::config_local_dir(),
            Self::Data => dirs::data_dir(),
            Self::DataLocal => dirs::data_local_dir(),
            Self::Executable => dirs::executable_dir(),
            Self::Desktop => dirs::desktop_dir(),
        }
    }
}
