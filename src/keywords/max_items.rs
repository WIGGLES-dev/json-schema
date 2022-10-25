use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MaxItemsKeyword(pub serde_json::Number);

impl super::Keyword for MaxItemsKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
