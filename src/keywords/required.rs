use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequiredKeyword(pub Vec<String>);

impl super::Keyword for RequiredKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
