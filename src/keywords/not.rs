use crate::schema::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NotKeyword(pub Box<JsonSchema>);

impl super::Keyword for NotKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {
        compiler.compile(&mut self.0);
    }
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
