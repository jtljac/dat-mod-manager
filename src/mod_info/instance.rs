use std::collections::HashMap;
use std::error::Error;
use std::fs;
use serde::{Serialize, Deserialize, Serializer};

use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::constants;
use crate::mod_info::game::Game;
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
    profile_path: PathBuf,

    game: String,

    #[serde(skip)]
    #[serde(default = "ModList::new")]
    mod_list: ModList,
}

impl Instance {
    pub fn new(base_path: PathBuf, mods_path: PathBuf, downloads_path: PathBuf, overwrite_path: PathBuf, profile_path: PathBuf, game: &str) -> Self {
        Self { base_path, mods_path, downloads_path, overwrite_path, profile_path, game: game.to_string(), mod_list: ModList::new() }
    }

    fn from_path(path: &Path) -> Result<Instance, Box<dyn Error>> {
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
            profile_path: PathBuf::from("profiles"),
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

    pub fn profile_path(&self) -> PathBuf {
        if self.profile_path.is_absolute() {
            self.profile_path.clone()
        } else {
            self.base_path.join(self.profile_path.as_path())
        }
    }

    pub fn game(&self) -> &str {
        &self.game
    }
}

/* ========================================= */
/* Util                                      */
/* ========================================= */

pub fn list_instances() -> HashMap<String, Instance> {
    let instance_path = constants::instance_dir();
    let mut profiles = HashMap::new();

    if !instance_path.exists() {
        return profiles
    }

    let files = fs::read_dir(&instance_path).unwrap_or_else(|err| {
        let path_string = instance_path.display();
        panic!("Failed to access instance folder at {path_string}\nError: {err}")
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