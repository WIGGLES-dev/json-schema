use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MinPropertiesKeyword(pub serde_json::Number);

impl super::Keyword for MinPropertiesKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
