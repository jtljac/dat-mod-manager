use std::path::PathBuf;
use directories::ProjectDirs;

const ORGANISATION: &str = "Dat Developer";
const NAME: &str = env!("CARGO_PKG_NAME");

pub fn config_dir() -> PathBuf {
    match ProjectDirs::from("com", ORGANISATION,NAME) {
        None => {
            panic!("Failed to access config directory");
        }
        Some(proj_dirs) => {
            proj_dirs.config_dir().to_path_buf()
        }
    }
}

pub fn config_file_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn instance_dir() -> PathBuf {
    config_dir().join("instances/")
}

pub fn instance_data_dir() -> PathBuf {
    data_dir().join("instances/")
}

pub fn data_dir() -> PathBuf {
    match ProjectDirs::from("com", ORGANISATION,NAME) {
        None => {
            panic!("Failed to access data directory");
        }
        Some(proj_dirs) => {
            return proj_dirs.data_dir().to_path_buf()
        }
    }
}