use std::collections::hash_map::Entry;
use std::collections::HashMap;
use error_chain::bail;
use crate::plugin::downloader::Downloader;
use crate::plugin::plugin_errors;

pub struct PluginRegister<T> {
    registry: HashMap<String, T>,
}

impl<T> Default for PluginRegister<T> {
    fn default() -> Self {
        PluginRegister{registry: HashMap::new()}
    }
}

impl<T> PluginRegister<T> {
    pub fn register(&mut self, id: &str, item: T) -> plugin_errors::Result<()> {
        match self.registry.entry(id.to_string()) {
            Entry::Occupied(_) => {
                bail!(plugin_errors::ErrorKind::RegistryIdExists)
            }
            Entry::Vacant(v) => {
                v.insert(item);
                Ok(())
            }
        }
    }

    pub fn deregister(&mut self, id: &str) {
        self.registry.remove(id);
    }

    pub fn get_registered(&self, key: &str) -> Option<&T> {
        self.registry.get(key)
    }
}

impl PluginRegister<Box<dyn Downloader>> {
    pub fn get_downloaders_for_protocol(&self, protocol: &str) -> Vec<&dyn Downloader> {
        self.registry.values()
            .filter(|downloader| downloader.as_ref().supported_protocols().contains(&protocol))
            .map(|downloader| downloader.as_ref())
            .collect()
    }
}