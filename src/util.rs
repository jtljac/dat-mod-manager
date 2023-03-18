use std::fs;
use dat_mod_manager::constants;

pub fn ensure_config_dir() {
    let config_path = constants::config_dir();
    if !config_path.exists() {
        fs::create_dir_all(config_path).unwrap();
    }

    let instance_path = constants::instance_dir();
    if !instance_path.exists() {
        fs::create_dir_all(instance_path).unwrap();
    }
}

pub fn ensure_data_dir() {
    let data_path = constants::data_dir();
    if !data_path.exists() {
        fs::create_dir_all(data_path).unwrap();
    }
}