use core::fmt;

use heapless::Vec;

#[derive(Clone, Debug)]
pub struct State {
    pub title: &'static str,
    pub attributes: Vec<Attribute, 64>,
    pub selected_attribute: usize,
    pub menu: Menu,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum Menu {
    Main,
    Sub,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum StateError {
    AttributesFull,
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

impl State {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            attributes: Vec::new(),
            selected_attribute: 0,
            menu: Menu::Main,
        }
    }

    pub fn with_attributes(mut self, attributes: &[Attribute]) -> Result<Self, StateError> {
        self.attributes
            .extend_from_slice(attributes)
            .map_err(|_| StateError::AttributesFull)?;
        Ok(self)
    }

    pub fn with_selected_attribute(mut self, selected_attribute: usize) -> Self {
        self.selected_attribute = selected_attribute;
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attribute {
    pub name: &'static str,
    pub value: Value,
}

impl Attribute {
    pub fn new(name: &'static str) -> Self {
        Attribute {
            name,
            value: Value::F32(ValueF32::new(0.0)),
        }
    }

    pub fn with_value_f32(mut self, value_f32: ValueF32) -> Self {
        self.value = Value::F32(value_f32);
        self
    }

    pub fn with_value_select(mut self, value_select: ValueSelect) -> Self {
        self.value = Value::Select(value_select);
        self
    }
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

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum ValueSelectError {
    AvailableFull,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ValueSelect {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ValueSelect({:?})", self.available[self.selected]);
    }
}

impl ValueSelect {
    pub fn new(available: &[&'static str]) -> Result<Self, ValueSelectError> {
        let mut value_select = Self {
            available: Vec::new(),
            selected: 0,
        };
        value_select
            .available
            .extend_from_slice(available)
            .map_err(|_| ValueSelectError::AvailableFull)?;
        Ok(value_select)
    }

    pub fn with_selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }
}

#[derive(Copy, Clone)]
pub struct ValueF32 {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub writter: fn(&mut dyn fmt::Write, f32),
}

impl fmt::Debug for ValueF32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "ValueF32(value: {}, min: {}, max: {}, step: {})",
            self.value, self.min, self.max, self.step
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ValueF32 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ValueF32(value: {}, min: {}, max: {}, step: {})",
            self.value,
            self.min,
            self.max,
            self.step
        );
    }
}

fn default_writter(destination: &mut dyn fmt::Write, value: f32) {
    let value = (value * 100.0) as u32;
    write!(destination, "{}%", value).unwrap();
}

impl ValueF32 {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            min: 0.0,
            max: 1.0,
            step: 0.01,
            writter: default_writter,
        }
    }
}

impl ValueF32 {
    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn with_min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn with_writter(mut self, writter: fn(&mut dyn fmt::Write, f32)) -> Self {
        self.writter = writter;
        self
    }
}
