use crate::{
    context::Compiler,
    schema::JsonSchema,
    validator::{Validator, ValidatorState},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AdditionalPropertiesKeyword(pub Box<JsonSchema>);

impl super::Keyword for AdditionalPropertiesKeyword {
    fn compile(&mut self, compiler: &mut Compiler) {
        compiler.compile(&mut self.0);
    }

    fn patch(&self, validator: Validator) {}

    fn validate(&self, mut validator: Validator) {
        if let Some(value) = &mut validator.value {
            match value {
                serde_json::Value::Null => {
                    validator.state = ValidatorState::Fail(format!("null is not an object"))
                }
                serde_json::Value::Bool(_) => {
                    validator.state = ValidatorState::Fail(format!("bool is not an object"))
                }
                serde_json::Value::Number(_) => {
                    validator.state = ValidatorState::Fail(format!("number is not an object"))
                }
                serde_json::Value::String(_) => {
                    validator.state = ValidatorState::Fail(format!("string is not an object"))
                }
                serde_json::Value::Array(_) => {
                    validator.state = ValidatorState::Fail(format!("array is not an object"))
                }
                serde_json::Value::Object(map) => {
                    for (k, v) in map {
                        match &validator.schema {
                            JsonSchema::Object(keywords) => {
                                let in_props = keywords
                                    .props
                                    .as_ref()
                                    .map(|props| props.map.contains_key(k))
                                    .unwrap_or(false);
                                if !in_props {
                                    let in_pat_props = keywords
                                        .pat_props
                                        .as_ref()
                                        .map(|pat_props| pat_props.map.keys().any(|k| false))
                                        .unwrap_or(false);
                                    if !in_pat_props {
                                        // validate the values that are additional with the provided schema.
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
