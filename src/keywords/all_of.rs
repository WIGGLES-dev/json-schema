use crate::schema::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[derive(Deserialize, Serialize)]
pub struct AllOfKeyword(pub Vec<JsonSchema>);

impl super::Keyword for AllOfKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {
        for (i, schema) in &mut self.0.iter_mut().enumerate() {
            compiler.compile_rel_key(|s| write!(s, "/allOf/{i}").unwrap(), schema);
        }
    }
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
