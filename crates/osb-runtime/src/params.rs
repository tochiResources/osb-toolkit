use crate::math::{Color4, Vec2};

/// The native control a configurable maps to in the host.
#[derive(Debug, Clone, PartialEq)]
pub enum Control {
    Slider {
        min: f64,
        max: f64,
    },
    Checkbox,
    Color,
    Angle,
    Popup {
        choices: Vec<String>,
    },
    Point,
    Text,
    /// An asset picker resolving a host footage item / layer / asset.
    Picker {
        asset_kind: String,
    },
}

/// A parameter's default value.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue {
    Number(f64),
    Bool(bool),
    Color(Color4),
    Text(String),
    Point(Vec2),
    Choice(i32),
}

/// A host-facing parameter declaration derived from a `[Configurable]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDescriptor {
    pub name: String,
    pub label: String,
    pub group: Option<String>,
    pub description: Option<String>,
    pub control: Control,
    pub default: ParameterValue,
}

impl ParameterDescriptor {
    pub fn new(name: impl Into<String>, control: Control, default: ParameterValue) -> Self {
        let name = name.into();
        let label = humanize(&name);
        ParameterDescriptor {
            label,
            name,
            group: None,
            description: None,
            control,
            default,
        }
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

fn humanize(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    for (index, ch) in name.chars().enumerate() {
        if index > 0 && ch.is_ascii_uppercase() {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn humanizes_camel_case() {
        let descriptor = ParameterDescriptor::new(
            "BarCount",
            Control::Slider {
                min: 0.0,
                max: 64.0,
            },
            ParameterValue::Number(0.0),
        );
        assert_eq!(descriptor.label, "Bar Count");
    }
}
