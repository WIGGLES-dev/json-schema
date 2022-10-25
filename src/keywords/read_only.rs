use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReadOnlyKeyword(pub bool);

impl super::Keyword for ReadOnlyKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
