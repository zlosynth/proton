use alloc::vec::Vec;

pub struct Store {
    pub modules: Vec<Module>,
    pub selected_module: usize,
}

pub struct Module {
    pub name: &'static str,
    pub index: usize,
    pub consumers: Vec<Consumer>,
}

pub struct Consumer {
    pub name: &'static str,
}
