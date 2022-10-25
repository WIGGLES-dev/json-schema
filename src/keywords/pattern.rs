use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PatternKeyword(pub String);

impl super::Keyword for PatternKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
