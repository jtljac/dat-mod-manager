use super::game_mod::ModTrait;

type ModReference = Box<dyn ModTrait>;

pub(crate) struct ModList {
    mods: Vec<ModReference>
}

impl ModList {
    pub fn new() -> Self {
        Self { mods: vec![] }
    }
}