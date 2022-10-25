use crate::pointer::JsonPointer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DynamicRefKeyword(pub JsonPointer);

impl super::Keyword for DynamicRefKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
