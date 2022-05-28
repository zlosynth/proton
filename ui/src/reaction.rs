#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reaction {
    SetValue(&'static str, f32),
    SelectValue(&'static str, &'static str),
}
