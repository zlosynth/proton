#[derive(Clone, Copy, PartialEq)]
pub enum Reaction<PI, CI> {
    RemovePatch(PI, CI),
}
