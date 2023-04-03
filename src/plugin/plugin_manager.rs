use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use error_chain::bail;
use libloading::{Library, Symbol};
use crate::constants::plugins_dir;
use crate::plugin::plugin_errors::ResultExt;
use crate::plugin::{Plugin, plugin_errors};
use crate::plugin::downloader::Downloader;
use crate::plugin::plugin_manager::plugin_register::PluginRegister;

pub mod plugin_register;
pub mod built_in_plugins;

#[derive(Default)]
pub struct PluginManager {
    plugins: HashMap<String, Lib>,
    registry: PluginRegistry
}

impl PluginManager {
    pub fn register_plugins(&mut self) {
        self.register_internal_plugins();

        let plugins_dir = plugins_dir();
        if plugins_dir.exists() {
            for entry in fs::read_dir(plugins_dir).unwrap() {
                match entry {
                    Ok(entry) => {
                        unsafe {
                            self.load_plugin(entry.file_name()).expect("Failed to load plugin}");
                        }
                    }
                    Err(err) => {
                        println!("Failed to access file:\n{err}")
                    }
                }
            }
        }
    }

    fn register_internal_plugins(&mut self) {
        // let http_downloader = Box::new(http_downloader::HttpDownloader {});
        // self.registry.downloaders.register(http_downloader.as_ref().name(), http_downloader).expect("Failed to add http downloader");
    }

    unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> plugin_errors::Result<()> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let lib = Library::new(&filename.as_ref()).chain_err(|| "Unable to load plugin")?;

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")
            .chain_err(|| "The `_plugin_create` symbol wasn't found")?;
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);

        let plugin_id = plugin.name().to_string();
        match self.plugins.entry(plugin_id) {
            Entry::Occupied(_) => {
                println!("Library {} tried to register a plugin with the id {plugin_id}, but a plugin \
                with that Id already exists.", filename.as_ref().to_str().unwrap());
                bail!(plugin_errors::ErrorKind::PluginIdExists)
            }
            Entry::Vacant(v) => {
                v.insert(Lib::new(lib, plugin));
            }
        }

        dbg!("Loaded plugin: {}", plugin.name());
        plugin.register_components(&mut self.registry);

        Ok(())
    }

    pub fn unload(&mut self) {
        dbg!("Unloading plugins");

        let drain = self.plugins.drain();
        for plugin in drain {
            dbg!("Firing on_plugin_unload for {:?}", plugin.0);
            plugin.1.plugin.unregister_components(&mut self.registry);
            drop(plugin.1.library);
        }
    }

    pub fn libraries(&self) -> &HashMap<String, Lib> {
        &self.plugins
    }
    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() {
            self.unload()
        }
    }
}

pub struct Lib {
    library: Library,
    plugin: Box<dyn Plugin>
}

impl Lib {
    pub fn new(library: Library, plugin: Box<dyn Plugin>) -> Self {
        Self { library, plugin }
    }
}

#[derive(Default)]
pub struct PluginRegistry {
    downloaders: PluginRegister<Box<dyn Downloader>>

}

impl PluginRegistry {
    pub fn downloaders(&mut self) -> &PluginRegister<Box<dyn Downloader>> {
        &self.downloaders
    }
}