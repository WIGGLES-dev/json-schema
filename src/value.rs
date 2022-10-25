use indextree::{Arena, NodeId};

pub enum Value {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Key(String),
    Object,
    Array,
    Root,
}

pub struct ValueArena {
    arena: Arena<Value>,
}

impl ValueArena {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }
}

impl From<serde_json::Value> for ValueArena {
    fn from(value: serde_json::Value) -> Self {
        let mut arena = Arena::new();
        unpack(&mut arena, value, None);
        Self { arena }
    }
}

impl Into<serde_json::Value> for ValueArena {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

fn unpack(values: &mut Arena<Value>, value: serde_json::Value, parent: Option<NodeId>) -> NodeId {
    let parent = parent.unwrap_or(values.new_node(Value::Root));
    match value {
        serde_json::Value::Null => {
            let id = values.new_node(Value::Null);
            parent.append(id, values);
            id
        }
        serde_json::Value::Bool(bool) => {
            let id = values.new_node(Value::Bool(bool));
            parent.append(id, values);
            id
        }
        serde_json::Value::Number(num) => {
            let id = values.new_node(Value::Number(num));
            parent.append(id, values);
            id
        }
        serde_json::Value::String(str) => {
            let id = values.new_node(Value::String(str));
            parent.append(id, values);
            id
        }
        serde_json::Value::Array(arr) => {
            let pid = values.new_node(Value::Array);
            parent.append(pid, values);
            for v in arr {
                unpack(values, v, Some(pid));
            }
            pid
        }
        serde_json::Value::Object(map) => {
            let pid = values.new_node(Value::Object);
            parent.append(pid, values);
            for (k, v) in map {
                let kid = values.new_node(Value::Key(k));
                pid.append(kid, values);
                unpack(values, v, Some(kid));
            }
            pid
        }
    }
}

fn pack() {}
