use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, PartialEq)]
pub struct State<NI, CI, PI> {
    pub view: View,
    pub modules: Vec<Module<NI, CI, PI>>,
    pub selected_module: usize,
    pub patches: Vec<Patch<CI, PI>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Modules,
    Patches,
}

impl<NI, CI, PI> Default for State<NI, CI, PI> {
    fn default() -> Self {
        Self {
            view: View::Patches,
            modules: vec![],
            selected_module: 0,
            patches: vec![],
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Module<NI, CI, PI> {
    pub handle: NI,
    pub name: &'static str,
    pub index: usize,
    pub attributes: Vec<Attribute<CI, PI>>,
    pub selected_attribute: usize,
}

#[derive(Clone, PartialEq)]
pub struct Attribute<CI, PI> {
    pub socket: Socket<CI, PI>,
    pub name: &'static str,
    pub connected: bool,
    pub value: &'static str,
}

#[derive(Clone, PartialEq)]
pub enum Socket<CI, PI> {
    Consumer(CI),
    Producer(PI),
}

impl<CI: Copy, PI: Copy> Socket<CI, PI> {
    pub fn consumer(&self) -> CI {
        if let Socket::Consumer(consumer) = self {
            *consumer
        } else {
            panic!();
        }
    }

    pub fn producer(&self) -> PI {
        if let Socket::Producer(producer) = self {
            *producer
        } else {
            panic!();
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Patch<CI, PI> {
    pub source: PI,
    pub source_module_name: &'static str,
    pub source_module_index: usize,
    pub source_attribute_name: &'static str,
    pub destination: CI,
    pub destination_module_name: &'static str,
    pub destination_module_index: usize,
    pub destination_attribute_name: &'static str,
}
