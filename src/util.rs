use std::fs;
use std::path::{PathBuf};

use crate::constants;

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

pub fn delete_dir_with_callback<F>(dir: PathBuf, mut callback: F)
where F: FnMut(u32, u32, &str) {
    let files: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(dir).contents_first(true)
        .into_iter().filter_map(|entry| entry.ok()).collect();

    for (index, entry) in files.iter().enumerate() {
        let path = entry.path();

        callback(files.len() as u32, (index + 1) as u32, path.file_name().unwrap().to_str().unwrap());
        if path.is_dir() {
            fs::remove_dir(path).expect("Failed to remove dir");
        } else {
            fs::remove_file(path).expect("Failed to remove file");
        }
    }
}