use std::collections::HashMap;

pub struct Game {
    name: String,
    description: String,
    categories: HashMap<u32, String>
}

impl Game {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn categories(&self) -> &HashMap<u32, String> {
        &self.categories
    }
}