use crate::validator::Validator;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DefaultKeyword(pub serde_json::Value);

impl super::Keyword for DefaultKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: Validator) {}
    fn validate(&self, validator: Validator) {}
}
