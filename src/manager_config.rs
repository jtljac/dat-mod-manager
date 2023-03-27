use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::errors::{ErrorKind, Result};
use crate::constants;


#[derive(Serialize, Deserialize)]
pub struct ManagerConfig {
    pub default_instance: String,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self{default_instance: "".to_string()}
    }
}

impl ManagerConfig {
    pub fn load() -> Result<Self> {
        let config_path = constants::config_file_path();
        let config_string = fs::read_to_string(config_path)?;

        let config = toml::from_str(&config_string)?;
        Ok(config)
    }

    pub fn load_or_create() -> ManagerConfig {
        match Self::load() {
            Ok(config) => config,
            Err(ref err) => {
                let config_path = constants::config_file_path();
                let config = ManagerConfig::default();

                match err.kind() {
                    ErrorKind::Io(e) => {
                        match e.kind() {
                            io::ErrorKind::NotFound => {
                                let config_string = toml::to_string(&config)
                                    .unwrap();
                                fs::write(config_path, config_string)
                                    .expect("Failed to write config file, using the default config");
                            }
                            io::ErrorKind::PermissionDenied => {
                                panic!("Permission denied when trying to access config file")
                            }
                            _ => {}
                        }
                    }
                    ErrorKind::Deserialise(_) => {
                        let mut new_path = config_path.with_extension(".bak");
                        let file_name = config_path.file_stem().unwrap().to_str().unwrap();
                        let mut i = 1;
                        while new_path.exists() {
                            new_path = new_path.with_file_name(format!("{file_name}{i}.bak"));
                            i += 1;
                        }
                        fs::rename(config_path, new_path).expect("Failed to rename bad config file");
                    }
                    _ => {}
                }

                config
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = constants::config_file_path();

        let config_string = toml::to_string(self)?;

        fs::write(config_path, config_string)?;

        Ok(())
    }

    pub fn quick_save(&self) {
        if let Err(ref e) = self.save() {
            eprintln!("Failed to save config:\n{e}");
        }
    }
}