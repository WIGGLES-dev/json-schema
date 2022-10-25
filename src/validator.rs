use crate::{context::Context, schema::JsonSchema};

#[derive(PartialEq)]
pub enum ValidatorState {
    Pass,
    Fail(String),
    Defer(String),
    NA,
}

pub struct ValidationError {
    message: &'static str,
}

pub struct Validator<'a> {
    pub parent: Option<&'a Validator<'a>>,
    pub errors: Vec<ValidationError>,
    pub state: ValidatorState,
    pub schema: &'a JsonSchema,
    pub root: &'a mut serde_json::Value,
    pub segment: &'a str,
    pub value: Option<&'a mut serde_json::Value>,
    pub context: &'a mut Context,
}

impl<'a> Validator<'a> {}
