use crate::{keywords::Keywords, pointer::JsonPointer};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonSchema {
    Ref {
        #[serde(rename = "$ref")]
        ref_: JsonPointer,
    },
    Mod {
        #[serde(rename = "$mod")]
        mod_: JsonPointer,
    },
    Bool(bool),
    Object(Keywords),
    Resolved(Url),
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResolvedJsonSchema {
    Bool(bool),
    Object(Keywords),
}
