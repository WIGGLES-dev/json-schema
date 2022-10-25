use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConstantKeyword(pub serde_json::Value);

impl super::Keyword for ConstantKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
