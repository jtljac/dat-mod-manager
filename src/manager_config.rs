use std::error::Error;
use std::{fs, io};
use serde::{Deserialize, Serialize};
use crate::constants;

#[derive(Serialize, Deserialize)]
pub struct ManagerConfig {
    pub default_instance: Option<String>,
}

impl ManagerConfig {
    pub fn default() -> ManagerConfig {
        Self{default_instance: None}
    }

    pub fn load() -> Result<ManagerConfig, Box<dyn Error>> {
        let config_path = constants::config_file_path();
        let config_string = fs::read_to_string(config_path)?;

        toml::from_str(&config_string)?
    }

    pub fn load_or_create() -> ManagerConfig {
        match Self::load() {
            Ok(config) => config,
            Err(ref err) => {
                let config_path = constants::config_file_path()

                if err.kind() == io::ErrorKind::PermissionDenied {
                    panic!("Permission denied when trying to access config file")
                } else {
                    if config_path.exists() {
                        let mut newPath = config_path.with_extension(".bak");
                        let mut i = 1;
                        while newPath.exists() {
                            newPath = newPath.with_file_name(config_path.file_stem() + i + ".bak");
                            i += 1;
                        }
                        fs::rename(config_path, newPath).expect("Failed to rename bad config file");
                    }

                    let config = ManagerConfig::default();
                    let config_string = toml::to_string(&config);
                    config
                }
            }
        }
    }
}