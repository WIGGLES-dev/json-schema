use crate::schema::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Write};

#[derive(Deserialize, Serialize)]
pub struct DefsKeyword {
    #[serde(flatten)]
    pub map: HashMap<String, JsonSchema>,
}

impl super::Keyword for DefsKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {
        for (k, v) in &mut self.map {
            compiler.compile_rel_key(|s| write!(s, "/$defs/{k}").unwrap(), v);
        }
    }
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
