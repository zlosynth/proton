#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct View {
    pub title: &'static str,
    pub attributes: [Option<Attribute>; 4],
    pub selected_attribute: usize,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attribute {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Value {
    Str(&'static str),
    F32(f32),
}
