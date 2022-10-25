//! Json Type Definition

use crate::schema::{JsonSchema, ResolvedJsonSchema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Boolean,
    String,
    Timestamp,
    Float32,
    Float64,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
}

#[derive(Deserialize, Serialize)]
pub struct TypeTypeDef {
    #[serde(rename = "type")]
    type_: Type,
}

#[derive(Deserialize, Serialize)]
pub struct EnumTypeDef(Vec<String>);

#[derive(Deserialize, Serialize)]
pub struct ElementsTypeDef(Box<TypeDefSchema>);

#[derive(Deserialize, Serialize)]
pub struct PropertiesTypeDef {
    properties: HashMap<String, TypeDefSchema>,
}

#[derive(Deserialize, Serialize)]
pub struct ValuesTypeDef {}

#[derive(Deserialize, Serialize)]
pub struct DiscriminatorTypeDef {}

#[derive(Deserialize, Serialize)]
pub struct UnionTypeDef {}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum TypeDef {
    Empty,
    Type(TypeTypeDef),
    Enum(EnumTypeDef),
    Elements(ElementsTypeDef),
    Properties(PropertiesTypeDef),
    Values(ValuesTypeDef),
    Discriminator(DiscriminatorTypeDef),
    Union(UnionTypeDef),
    Ref {
        #[serde(rename = "ref")]
        ref_: String,
    },
}

#[derive(Deserialize, Serialize)]
pub struct Metadata {}

#[derive(Deserialize, Serialize)]
pub struct TypeDefSchema {
    pub metadata: Metadata,
    #[serde(flatten)]
    pub schema: TypeDef,
}

#[derive(Deserialize, Serialize)]
pub struct RootTypeDefSchema {
    pub definitions: HashMap<String, TypeDefSchema>,
    #[serde(flatten)]
    pub schema: TypeDefSchema,
}

impl From<ResolvedJsonSchema> for RootTypeDefSchema {
    fn from(_: ResolvedJsonSchema) -> Self {
        let definitions = HashMap::new();
        let metadata = Metadata {};
        let schema = TypeDef::Empty;
        let schema = TypeDefSchema { metadata, schema };
        RootTypeDefSchema {
            definitions,
            schema,
        }
    }
}

impl Into<JsonSchema> for TypeDefSchema {
    fn into(self) -> JsonSchema {
        match self.schema {
            _ => JsonSchema::Bool(false),
        }
    }
}
