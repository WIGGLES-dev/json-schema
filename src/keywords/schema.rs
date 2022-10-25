use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SchemaKeyword(pub String);

impl super::Keyword for SchemaKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
