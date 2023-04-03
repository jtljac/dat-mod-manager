use std::any::Any;
use crate::plugin::plugin_manager::PluginRegistry;

pub mod plugin_errors;
pub mod plugin_util;

pub mod plugin_manager;

pub mod installer;
pub mod downloader;


pub trait Plugin: Any + Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn register_components(&self, registry: &mut PluginRegistry);
    fn unregister_components(&self, registry: &mut PluginRegistry);
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}