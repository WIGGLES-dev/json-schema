use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CommentKeyword(pub String);

impl super::Keyword for CommentKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {}
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
