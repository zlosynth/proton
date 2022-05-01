use alloc::vec::Vec;

pub struct Store {
    pub modules: Vec<Module>,
}

pub struct Module {
    pub name: &'static str,
}
