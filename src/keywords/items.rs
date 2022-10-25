use serde::{Deserialize, Serialize};

use crate::schema::JsonSchema;

#[derive(Deserialize, Serialize)]
pub struct ItemsKeyword(pub Box<JsonSchema>);

impl super::Keyword for ItemsKeyword {
    fn compile(&mut self, compiler: &mut crate::context::Compiler) {
        compiler.compile(&mut self.0);
    }
    fn patch(&self, validator: crate::validator::Validator) {}
    fn validate(&self, validator: crate::validator::Validator) {}
}
