pub trait ModTrait {
    fn get_name(&self) -> &String;
    fn get_author(&self) -> &String;
    fn get_version(&self) -> &String;
    fn get_categories(&self) -> &Vec<u32>;
}

struct GameMod {
    name: String,
    author: String,
    version: String,
    categories: Vec<u32>,
}

impl GameMod {
    pub fn new(name: String, author: String, version: String, categories: Vec<u32>) -> Self {
        Self { name, author, version, categories }
    }
}

impl ModTrait for GameMod {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_author(&self) -> &String {
        &self.author
    }

    fn get_version(&self) -> &String {
        &self.version
    }

    fn get_categories(&self) -> &Vec<u32> {
        &self.categories
    }
}
