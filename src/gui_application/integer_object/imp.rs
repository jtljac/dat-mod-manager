use std::cell::{Cell, RefCell};

use glib::{ParamSpec, ParamSpecInt, Value};
use gtk::glib;
use gtk::glib::{ParamSpecString, ParamSpecUInt};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

// Object holding the state
#[derive(Default)]
pub struct StringObject {
    name: RefCell<String>,
    pos: Cell<u32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for StringObject {
    const NAME: &'static str = "MyGtkAppIntegerObject";
    type Type = super::StringObject;
}

// Trait shared by all GObjects
impl ObjectImpl for StringObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder("name").build(), ParamSpecUInt::builder("pos").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let input_name =
                    value.get().expect("The value needs to be of type `string`.");
                self.name.replace(input_name);
            },
            "pos" => {
                let input_pos =
                    value.get().expect("The value needs to be of type `u32`.");
                self.pos.replace(input_pos);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "pos" => self.pos.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
