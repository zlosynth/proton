#[allow(unused_imports)]
use micromath::F32Ext;

use core::fmt;

use super::state;

const ATTRIBUTES_CAPACITY: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct View {
    pub title: &'static str,
    pub attributes: [Option<Attribute>; ATTRIBUTES_CAPACITY],
    pub selected_attribute: usize,
    pub edit: bool,
}

impl From<&state::State> for View {
    fn from(other: &state::State) -> Self {
        let attribute_page = selected_attribute_to_page(other.selected_attribute);
        let first_index = attribute_page * ATTRIBUTES_CAPACITY;
        Self {
            title: other.title,
            attributes: [
                other.attributes.get(first_index).map(|a| a.into()),
                other.attributes.get(first_index + 1).map(|a| a.into()),
                other.attributes.get(first_index + 2).map(|a| a.into()),
                other.attributes.get(first_index + 3).map(|a| a.into()),
            ],
            selected_attribute: other.selected_attribute % 4,
            edit: matches!(other.menu, state::Menu::Sub),
        }
    }
}

fn selected_attribute_to_page(selected_attribute: usize) -> usize {
    (selected_attribute as f32 / ATTRIBUTES_CAPACITY as f32).floor() as usize
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attribute {
    pub name: &'static str,
    pub value: Value,
}

impl From<&state::Attribute> for Attribute {
    fn from(other: &state::Attribute) -> Self {
        #[allow(clippy::needless_borrow)] // It's not needless, it fails without it
        Self {
            name: other.name,
            value: (&other.value).into(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Value {
    Str(&'static str),
    F32(fn(&mut dyn fmt::Write, f32), f32),
}

impl From<&state::Value> for Value {
    fn from(other: &state::Value) -> Self {
        match other {
            state::Value::Select(value_select) => {
                Value::Str(value_select.available[value_select.selected])
            }
            state::Value::F32(value_f32) => Value::F32(value_f32.writter, value_f32.value),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Str(value) => write!(fmt, "Value::Str({})", value),
            Self::F32(_, value) => write!(fmt, "Value::F32({})", value),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Value {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::Str(value) => defmt::write!(fmt, "Value::Str({})", value),
            Self::F32(_, value) => defmt::write!(fmt, "Value::F32({})", value),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Str(a), Self::Str(b)) => a == b,
            (Self::F32(_, a), Self::F32(_, b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_state_when_converted_into_view_it_should_provide_expected_result() {
        use crate::state;

        fn test_writter(destination: &mut dyn fmt::Write, value: f32) {
            write!(destination, "x{:.2}", value).unwrap();
        }

        let source_state = state::State::new("Title")
            .with_attributes(&[
                state::Attribute::new("a1").with_value_f32(state::ValueF32::new(1.0)),
                state::Attribute::new("a2").with_value_f32(state::ValueF32::new(1.0)),
                state::Attribute::new("a3").with_value_f32(state::ValueF32::new(1.0)),
                state::Attribute::new("a4").with_value_f32(state::ValueF32::new(1.0)),
                state::Attribute::new("a5").with_value_select(
                    state::ValueSelect::new(&["v1", "v2"])
                        .unwrap()
                        .with_selected(1),
                ),
                state::Attribute::new("a6")
                    .with_value_f32(state::ValueF32::new(1.0).with_writter(test_writter)),
            ])
            .unwrap()
            .with_selected_attribute(5);

        let expected_view = View {
            title: "Title",
            attributes: [
                Some(Attribute {
                    name: "a5",
                    value: Value::Str("v2"),
                }),
                Some(Attribute {
                    name: "a6",
                    value: Value::F32(test_writter, 1.0),
                }),
                None,
                None,
            ],
            selected_attribute: 1,
            edit: false,
        };

        let actual_view: View = (&source_state).into();
        assert_eq!(actual_view, expected_view);
    }
}
