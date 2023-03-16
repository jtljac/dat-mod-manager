use std::path::{Path, PathBuf};
use crate::mod_info::game::Game;
use crate::mod_info::mod_list::ModList;

struct Instance {
    base_path: PathBuf,
    mods_path: PathBuf,
    downloads_path: PathBuf,
    overwrite_path: PathBuf,
    profile_path: PathBuf,
    game: Game,
    mod_list: ModList
}

impl Instance {
    pub fn new(base_path: PathBuf, mods_path: PathBuf, downloads_path: PathBuf, overwrite_path: PathBuf, profile_path: PathBuf, game: Game, mod_list: ModList) -> Self {
        Self { base_path, mods_path, downloads_path, overwrite_path, profile_path, game, mod_list }
    }

    pub fn new_default(base_path: PathBuf, game: Game, ) -> Self {
        Self {
            base_path,
            mods_path: PathBuf::from("./mods"),
            downloads_path: PathBuf::from("./downloads"),
            overwrite_path: PathBuf::from("./overwrite"),
            profile_path: PathBuf::from("./profiles"),
            game,
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
}