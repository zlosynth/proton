use heapless::Vec;

#[derive(Clone, Debug)]
pub struct State {
    pub title: &'static str,
    pub attributes: Vec<Attribute, 64>,
    pub selected_attribute: usize,
}

#[cfg(feature = "defmt")]
impl defmt::Format for State {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "State{{{:?}, [", self.title);
        for attribute in self.attributes.iter() {
            defmt::write!(fmt, "  {:?},", attribute);
        }
        defmt::write!(fmt, "]}}");
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attribute {
    pub name: &'static str,
    pub value: Value,
}

#[allow(clippy::large_enum_variant)] // TODO: Use Box instead
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Value {
    Select(ValueSelect),
    F32(ValueF32),
}

#[derive(Clone, Debug)]
pub struct ValueSelect {
    pub available: Vec<&'static str, 12>,
    pub selected: usize,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ValueSelect {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ValueSelect({:?})", self.available[self.selected]);
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ValueF32 {
    pub value: f32,
    pub step: f32,
}
