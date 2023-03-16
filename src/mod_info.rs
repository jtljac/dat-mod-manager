pub mod game;
pub mod game_mod;
pub mod mod_list;
pub mod instance;

#[cfg(test)]
mod tests {
    use crate::mod_info::mod_list::ModList;

    #[test]
    fn mod_list_tests() {
        let list = ModList::new();

    }
}