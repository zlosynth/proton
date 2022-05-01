use alloc::vec::Vec;

pub struct Store {
    pub modules: Vec<Module>,
    pub modules_page: usize,
    pub selected_module: usize,
}

pub struct Module {
    pub name: &'static str,
    pub index: usize,
}
