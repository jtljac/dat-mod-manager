pub mod installer;

pub trait Plugin {
    fn name(&self) -> &'static str;

}