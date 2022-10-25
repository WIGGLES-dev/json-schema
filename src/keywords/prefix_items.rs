use crate::schema::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[derive(Deserialize, Serialize)]
pub struct PrefixItemsKeyword(pub Vec<JsonSchema>);

impl super::Keyword for PrefixItemsKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {
        for (i, schema) in &mut self.0.iter_mut().enumerate() {
            compiler.compile_rel_key(|s| write!(s, "/prefixItems/{i}").unwrap(), schema);
        }
    }
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
