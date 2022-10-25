use crate::schema::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    error,
    fs::{read_dir, File},
};

static DRAFT_2020_12: &'static str = "./JSON-Schema-Test-Suite/tests/draft2020-12";

#[derive(Deserialize, Serialize)]
pub struct JsonSchemaTestData {
    pub description: String,
    pub data: serde_json::Value,
    pub valid: bool,
}

impl JsonSchemaTestData {
    pub fn test(&self, schema: &JsonSchema) -> bool {
        true
    }
}

#[derive(Deserialize, Serialize)]
pub struct JsonSchemaTest {
    pub description: String,
    pub schema: JsonSchema,
    pub tests: Vec<JsonSchemaTestData>,
}

#[test]
fn test_draft_2020_12() -> Result<(), Box<dyn error::Error>> {
    for dirent in read_dir(DRAFT_2020_12)? {
        let dirent = dirent?;
        if dirent.path().is_file() {
            let mut file = File::open(dirent.path()).expect("invalid path");
            let tests: Vec<JsonSchemaTest> = serde_json::from_reader(&mut file)?;
            for (i, test) in tests.iter().enumerate() {
                println!("{:#?} [{}]", dirent.path(), i);
                println!("{}", &test.description);
                println!("{}", serde_yaml::to_string(&test.schema)?);
                for test_data in &test.tests {
                    if test_data.test(&test.schema) == false {
                        panic!("test failed");
                    }
                }
            }
        }
    }
    Ok(())
}
