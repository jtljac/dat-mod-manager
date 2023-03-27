use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use error_chain::bail;

use crate::errors;
use crate::constants::instance_dir;
use crate::errors::ErrorKind;
use crate::mod_info::mod_list::ModList;

/* ========================================= */
/* Instance                                  */
/* ========================================= */

#[derive(Deserialize, Serialize)]
pub struct Instance {
    base_path: PathBuf,
    mods_path: PathBuf,
    downloads_path: PathBuf,
    overwrite_path: PathBuf,
    profiles_path: PathBuf,

    game: String,

    #[serde(skip)]
    #[serde(default = "ModList::new")]
    mod_list: ModList,
}

impl Instance {
    pub fn new(base_path: &Path, mods_path: &Path, downloads_path: &Path, overwrite_path: &Path, profile_path: &Path, game: &str) -> Self {
        Self { base_path: base_path.to_path_buf(), mods_path: mods_path.to_path_buf(), downloads_path: downloads_path.to_path_buf(), overwrite_path: overwrite_path.to_path_buf(), profiles_path: profile_path.to_path_buf(), game: game.to_string(), mod_list: ModList::new() }
    }

    pub fn from_name(profile_name: &str) -> errors::Result<Instance> {
        let profile_path = instance_dir().join(profile_name.to_string() + ".toml");

        Instance::from_path(&profile_path)
    }

    fn from_path(path: &Path) -> errors::Result<Instance> {
        let toml_content = fs::read_to_string(path)?;

        let instance = toml::from_str(&toml_content)?;

        Ok(instance)
    }

    pub fn new_default(base_path: PathBuf, game: &str) -> Self {
        Self {
            base_path,
            mods_path: PathBuf::from("mods"),
            downloads_path: PathBuf::from("downloads"),
            overwrite_path: PathBuf::from("overwrite"),
            profiles_path: PathBuf::from("profiles"),
            game: game.to_string(),
            mod_list: ModList::new()
        }
    }

    pub fn base_path(&self) -> &Path {
        self.base_path.as_path()
    }

    pub fn mods_path(&self) -> PathBuf {
        if self.mods_path.is_absolute() {
            self.mods_path.clone()
        } else {
            self.base_path.join(self.mods_path.as_path())
        }
    }

    pub fn downloads_path(&self) -> PathBuf {
        if self.downloads_path.is_absolute() {
            self.downloads_path.clone()
        } else {
            self.base_path.join(self.downloads_path.as_path())
        }
    }

    pub fn overwrite_path(&self) -> PathBuf {
        if self.overwrite_path.is_absolute() {
            self.overwrite_path.clone()
        } else {
            self.base_path.join(self.overwrite_path.as_path())
        }
    }

    pub fn profiles_path(&self) -> PathBuf {
        if self.profiles_path.is_absolute() {
            self.profiles_path.clone()
        } else {
            self.base_path.join(self.profiles_path.as_path())
        }
    }

    pub fn game(&self) -> &str {
        &self.game
    }
}

/* ========================================= */
/* Util                                      */
/* ========================================= */

/// Get a list of all the registered instances
///
/// returns: HashMap<String, Instance> A map with the instance name as the key and the instance as the value
///
pub fn get_instances() -> HashMap<String, Instance> {
    let instance_path = instance_dir();
    let mut profiles = HashMap::new();

    if !instance_path.exists() {
        return profiles
    }

    let files = fs::read_dir(&instance_path).unwrap_or_else(|err| {
        panic!("Failed to access instance folder at {}\nError: {err}", instance_path.display())
    });

    for path in files {
        let file = path.unwrap().path();
        if file.is_file() {
            let instance_name = file.file_stem().unwrap().to_str().unwrap();
            let instance_file = match Instance::from_path(file.as_path()) {
                Ok(file) => file,
                Err(err) => {
                    println!("Failed to load instance file for instance: {instance_name}\nError: {err}");
                    continue;
                }
            };
            profiles.insert(instance_name.to_string(), instance_file);
        }
    }

    profiles
}

pub fn list_instances() -> Vec<String> {
    let instance_path = instance_dir();
    let mut profiles = Vec::new();

    if !instance_path.exists() {
        return profiles
    }

    let files = fs::read_dir(&instance_path).unwrap_or_else(|err| {
        panic!("Failed to access instance folder at {}\nError: {err}", instance_path.display())
    });

    for path in files {
        let file = path.unwrap().path();
        if file.is_file() {
            let instance_name = file.file_stem().unwrap().to_str().unwrap();

            profiles.push(instance_name.to_string());
        }
    }

    profiles
}

pub fn create_instance(name: &str, game: &str, base_path: &Path, mods_path: &Path, downloads_path: &Path, overwrite_path: &Path, profile_path: &Path) -> errors::Result<Instance> {
    if get_instances().contains_key(name) {bail!(ErrorKind::InstanceExists)}

    let instance = Instance::new(base_path, mods_path, downloads_path, overwrite_path, profile_path, game);

    if !instance.base_path().exists() {
        fs::create_dir_all(instance.base_path())?
    }

    if !instance.mods_path().exists() {
        fs::create_dir_all(instance.mods_path())?
    }

    if !instance.downloads_path().exists() {
        fs::create_dir_all(instance.downloads_path())?
    }

    if !instance.overwrite_path().exists() {
        fs::create_dir_all(instance.overwrite_path())?
    }

    if !instance.profiles_path().exists() {
        fs::create_dir_all(instance.profiles_path())?
    }

    fs::write(instance_dir().join(name.to_string() + ".toml"), toml::to_string(&instance)?)?;

    Ok(instance)
}

pub fn delete_instance(name: &str) -> errors::Result<()>{
    match fs::remove_file(instance_dir().join(name.to_string() + ".toml")) {
        Ok(_) => Ok(()),
        Err(err) => bail!(err)
    }
}

