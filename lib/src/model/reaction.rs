#[derive(Clone, Copy, PartialEq)]
pub enum Reaction<PI, CI> {
    ConnectPatch(PI, CI),
    RemovePatch(PI, CI),
}
