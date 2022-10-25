use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IfThenElseKeyword {
    if_: (),
    then: (),
    else_: (),
}

impl super::Keyword for IfThenElseKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
