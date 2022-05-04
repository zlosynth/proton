use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, PartialEq)]
pub struct State<NI, CI, PI> {
    pub view: View,
    pub modules: Vec<Module<NI, CI, PI>>,
    pub selected_module: usize,
    pub patches: Vec<Patch<CI, PI>>,
    pub selected_patch: usize,
    // TODO:
    // Add view of patch edit, that would hide all other destinations and all arrows, left stays highlighted
    // On right side, the source that was selected keeps the arrow
    // Right encoder scrolls through pages of sources, highlighting them
    // Clicking or moving left encoder leaves the edit
    // Clicking the right encoder selects source
    pub patch_edit: bool,
    pub patch_edit_sources: Vec<Source<PI>>,
    pub patch_edit_selected_source: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Modules,
    Patches,
}

impl<NI, CI, PI> Default for State<NI, CI, PI> {
    fn default() -> Self {
        Self {
            view: View::Modules,
            modules: vec![],
            selected_module: 0,
            patches: vec![],
            selected_patch: 0,
            // TODO
            patch_edit: false,
            patch_edit_sources: vec![],
            patch_edit_selected_source: 0,
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
    pub source: Option<Source<PI>>,
    pub destination: Option<Destination<CI>>,
}

#[derive(Clone, PartialEq)]
pub struct Source<PI> {
    pub producer: PI,
    pub module_name: &'static str,
    pub module_index: usize,
    pub attribute_name: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct Destination<CI> {
    pub consumer: CI,
    pub module_name: &'static str,
    pub module_index: usize,
    pub attribute_name: &'static str,
}
