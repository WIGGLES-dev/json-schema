mod array;
mod boolean;
mod def;
mod map;
mod number;
mod string;

use serde::{de::DeserializeSeed, Deserialize};
use serde_yaml::value::Tag;
use std::collections::HashMap;

pub struct MacroOps {
    /// mapping of yaml tag names to macro expansion definitions
    ops: HashMap<Tag, fn(serde_yaml::Value) -> serde_json::Value>,
}

impl MacroOps {
    pub fn new() -> Self {
        let ops = HashMap::new();
        Self { ops }
    }
    pub fn insert(
        &mut self,
        tag: impl Into<String>,
        f: fn(serde_yaml::Value) -> serde_json::Value,
    ) {
        self.ops.insert(Tag::new(tag), f);
    }
}

impl<'de> DeserializeSeed<'de> for MacroOps {
    type Value = serde_json::Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let yaml_val = serde_yaml::value::Value::deserialize(deserializer)?;
        match yaml_val {
            serde_yaml::Value::Tagged(tag) => {
                if let Some(transform) = self.ops.get(&tag.tag) {
                    return Ok((transform)(tag.value));
                } else {
                    self.deserialize(serde_yaml::Value::Tagged(tag))
                        .map_err(serde::de::Error::custom)
                }
            }
            v => serde_json::Value::deserialize(v).map_err(serde::de::Error::custom),
        }
    }
}

fn format_macro(value: serde_yaml::Value) -> serde_json::Value {
    let formatted = String::new();
    serde_json::Value::String(formatted)
}

fn string_macro(value: serde_yaml::Value) -> serde_json::Value {
    serde_json::json!({
        "type": "string"
    })
}

fn ref_macro(value: serde_yaml::Value) -> serde_json::Value {
    serde_json::json!({
       "$ref": ""
    })
}

fn test() {
    let test_yaml = br#"
        deeply:
            nested:
                macro:
                    format! "a/b/c/{value}"
    "#;
    let mut ops = MacroOps::new();
    ops.insert("Format", format_macro);
    let parsed = ops
        .deserialize(serde_yaml::from_slice::<serde_yaml::Value>(test_yaml).unwrap())
        .unwrap();
}
