use alloc::vec::Vec;

pub struct Store {
    pub modules: Vec<Module>,
    pub selected_module: usize,
}

pub struct Module {
    pub name: &'static str,
    pub index: usize,
    pub attributes: Vec<Attribute>,
    pub selected_attribute: usize,
}

pub struct Attribute {
    pub name: &'static str,
}
