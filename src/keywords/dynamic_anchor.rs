use crate::pointer::JsonPointer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DynamicAnchorKeyword(pub JsonPointer);

impl super::Keyword for DynamicAnchorKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
