use glib::Object;
use gtk::glib;
use gtk::glib::ObjectExt;

mod imp;

glib::wrapper! {
    pub struct StringObject(ObjectSubclass<imp::StringObject>);
}

impl StringObject {
    pub fn new(value: &str, pos: u32) -> Self {
        Object::builder().property("name", value).property("pos", pos).build()
    }

    pub fn set_pos(self, new_pos: u32) {
        self.set_property("pos", new_pos);
    }
}
