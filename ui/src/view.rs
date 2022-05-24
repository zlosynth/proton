#[allow(unused_imports)]
use micromath::F32Ext;

use super::state;

const ATTRIBUTES_CAPACITY: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct View {
    pub title: &'static str,
    pub attributes: [Option<Attribute>; ATTRIBUTES_CAPACITY],
    pub selected_attribute: usize,
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
        Self {
            name: other.name,
            value: (&other.value).into(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Value {
    Str(&'static str),
    F32(f32),
}

impl From<&state::Value> for Value {
    fn from(other: &state::Value) -> Self {
        match other {
            state::Value::Select(value_select) => {
                Value::Str(value_select.available[value_select.selected])
            }
            state::Value::F32(value) => Value::F32(*value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_state_when_converted_into_view_it_should_provide_expected_result() {
        use crate::state;
        use heapless::Vec;

        let source_state = state::State {
            title: "Title",
            attributes: Vec::from_slice(&[
                state::Attribute {
                    name: "a1",
                    value: state::Value::F32(1.0),
                },
                state::Attribute {
                    name: "a2",
                    value: state::Value::F32(1.0),
                },
                state::Attribute {
                    name: "a3",
                    value: state::Value::F32(1.0),
                },
                state::Attribute {
                    name: "a4",
                    value: state::Value::F32(1.0),
                },
                state::Attribute {
                    name: "a5",
                    value: state::Value::Select(state::ValueSelect {
                        available: Vec::from_slice(&["v1", "v2"]).unwrap(),
                        selected: 1,
                    }),
                },
                state::Attribute {
                    name: "a6",
                    value: state::Value::F32(1.0),
                },
            ])
            .unwrap(),
            selected_attribute: 5,
        };

        let expected_view = View {
            title: "Title",
            attributes: [
                Some(Attribute {
                    name: "a5",
                    value: Value::Str("v2"),
                }),
                Some(Attribute {
                    name: "a6",
                    value: Value::F32(1.0),
                }),
                None,
                None,
            ],
            selected_attribute: 1,
        };

        let actual_view: View = (&source_state).into();
        assert_eq!(actual_view, expected_view);
    }
}
