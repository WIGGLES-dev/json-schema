//! NOTES:
//!     - in complex schemas there is not way to infer good names for types, perhaps whenever
//!       there is not good name inferable from the url of a schemas or the schema itself we can
//!       fall back to a pattern of interface merging to keep the generated code clean.

use crate::{
    context::Context,
    keywords::{FormatKeyword, Keywords, TypeKeyword},
    schema::{JsonSchema, ResolvedJsonSchema},
};
use case_utils::Case;
use serde_json::Value;
use std::{
    collections::HashMap,
    io::{self, Write},
};
use url::Url;

mod resolve_ident;

struct CodegenState<'a> {
    pub context: &'a Context,
    idents: HashMap<Url, String>,
}

impl<'a> CodegenState<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            idents: HashMap::new(),
        }
    }
}

fn get_url(schema: &JsonSchema) -> &Url {
    match schema {
        JsonSchema::Resolved(url) => url,
        _ => panic!("all json schemas should be resolved"),
    }
}

fn is_number(str: &str) -> bool {
    str.chars().all(|c| c.is_numeric())
}

fn resolve_url_file_path_ident(url: &Url) -> String {
    let path = url.path();
    let path = std::path::Path::new(path);
    let ident = path
        .file_stem()
        .and_then(|os_str| os_str.to_str())
        .expect("valid utf-8 path");
    Case::Pascal.convert(ident)
}

fn resolve_url_ident(url: &Url) -> String {
    let path_ident = resolve_url_file_path_ident(url);

    if let Some(fragment) = url.fragment() {
        if let Some(last) = fragment.split("/").last() {
            if is_number(last) {
                path_ident
            } else {
                Case::Pascal.convert(last)
            }
        } else {
            path_ident
        }
    } else {
        path_ident
    }
}

fn resolve_rust_ident<'a>(
    state: &'a mut CodegenState,
    keywords: &Keywords,
    url: &Url,
) -> &'a String {
    if !state.idents.contains_key(url) {
        let title = &keywords.title;
        let ident = match title {
            Some(title) => Case::Pascal.convert(&title.0),
            _ => resolve_url_ident(url),
        };
        state.idents.insert(url.clone(), ident);
    }
    state.idents.get(url).unwrap()
}

/// Resolve a rust ident from previously generated idents. Some scalar types like Uuid and number are resolved idiomatically while
/// others are new types.
fn resolve_rust_type(
    w: &mut impl Write,
    state: &mut CodegenState,
    schema: &JsonSchema,
) -> std::io::Result<()> {
    let url = get_url(schema);
    if let Some(ident) = state.idents.get(url) {
        write!(w, "{ident}")?;
    } else {
        let schema = state.context.schemas.get(url);
        match schema {
            Some(ResolvedJsonSchema::Bool(bool)) => {
                if *bool {
                    write!(w, "serde_json::Value")?
                } else {
                    write!(w, "()")?
                }
            }
            Some(ResolvedJsonSchema::Object(keywords)) => match keywords {
                // resolve scalar types (i.e schemas with a single type that isn't object)
                Keywords {
                    type_: Some(TypeKeyword::Single(type_)),
                    format,
                    ..
                } => match (type_.as_str(), format) {
                    ("string", Some(FormatKeyword(format))) => match format.as_str() {
                        "date-time" => write!(w, "String")?,
                        "time" => write!(w, "String")?,
                        "date" => write!(w, "String")?,
                        "duration" => write!(w, "String")?,
                        "email" => write!(w, "String")?,
                        "idn-email" => write!(w, "String")?,
                        "host-name" => write!(w, "String")?,
                        "idn-host-name" => write!(w, "String")?,
                        "ipv4" => write!(w, "String")?,
                        "ipv6" => write!(w, "String")?,
                        "uuid" => write!(w, "uuid::Uuid")?,
                        "uri" | "url" => write!(w, "url::Url")?,
                        "uri-reference" => write!(w, "String")?,
                        "iri" => write!(w, "String")?,
                        "iri-reference" => write!(w, "String")?,
                        "uri-template" => write!(w, "String")?,
                        "json-pointer" => write!(w, "String")?,
                        "relative-json-pointer" => write!(w, "String")?,
                        "regex" => write!(w, "String")?,
                        _ => panic!("unsupported string format"),
                    },
                    ("string", None) => write!(w, "String")?,
                    ("number", Some(FormatKeyword(format))) => match format.as_str() {
                        "f32" => write!(w, "f32")?,
                        "f64" => write!(w, "f64")?,
                        _ => panic!("unsupported number format"),
                    },
                    ("number", None) => write!(w, "f32")?,
                    ("integer", Some(FormatKeyword(format))) => match format.as_str() {
                        "i8" => write!(w, "i8")?,
                        "u8" => write!(w, "u8")?,
                        "i16" => write!(w, "i16")?,
                        "u16" => write!(w, "u16")?,
                        "i32" => write!(w, "i32")?,
                        "u32" => write!(w, "u32")?,
                        _ => panic!("unsupported integer format"),
                    },
                    ("integer", None) => write!(w, "i32")?,
                    ("object", _) => {
                        let ident = resolve_rust_ident(state, keywords, url);
                        write!(w, "{ident}")?;
                    }
                    ("array", _) => match keywords {
                        Keywords {
                            items: Some(items),
                            prefix_items: None,
                            ..
                        } => {
                            write!(w, "Vec<")?;
                            resolve_rust_type(w, state, &items.0)?;
                            write!(w, ">")?;
                        }
                        Keywords {
                            items: None,
                            prefix_items: Some(prefix_items),
                            ..
                        } => {
                            write!(w, "(")?;
                            let schemas = &mut prefix_items.0.iter().peekable();
                            while let Some(schema) = schemas.next() {
                                resolve_rust_type(w, state, schema)?;
                                if schemas.peek().is_some() {
                                    write!(w, ", ")?;
                                }
                            }
                            write!(w, ")")?;
                        }
                        Keywords {
                            items: Some(items),
                            prefix_items: Some(prefix_items),
                            ..
                        } => {
                            write!(w, "Vec<()>")?;
                        }
                        _ => {
                            write!(w, "Vec<()>")?;
                        }
                    },
                    _ => panic!("unexpected type"),
                },
                _ => {
                    let ident = resolve_rust_ident(state, keywords, url);
                    write!(w, "{ident}")?;
                }
            },
            None => {
                write!(w, "()")?;
            }
        }
    }
    Ok(())
}

fn write_variants(w: &mut impl Write, state: &mut CodegenState) -> io::Result<()> {
    Ok(())
}

fn write_fields(
    w: &mut impl Write,
    state: &mut CodegenState,
    keywords: &Keywords,
) -> io::Result<()> {
    if let Some(props) = &keywords.props {
        let required = &vec![];
        let required = keywords.required.as_ref().map(|k| &k.0).unwrap_or(required);
        for (property_name, property_schema) in &props.map {
            let url = get_url(property_schema);
            // writeln!(w, "\t/// {url}")?;
            write!(w, "\tpub {property_name}: ")?;
            let is_required = required.iter().any(|p| p == property_name);
            if !is_required {
                write!(w, "Option<")?;
            }
            resolve_rust_type(w, state, property_schema)?;
            if !is_required {
                write!(w, ">")?;
            }
            writeln!(w, ",")?;
        }
    }
    if let Some(all_of) = &keywords.all_of {
        for schema in &all_of.0 {
            let url = get_url(schema);
            if let Some(ResolvedJsonSchema::Object(keywords)) = state.context.schemas.get(url) {
                let ident = resolve_rust_ident(state, keywords, url);
                writeln!(w, "\t/// {url}")?;
                writeln!(w, "\t#[serde(flatten)]")?;
                write!(w, "\tpub {ident}: ")?;
                resolve_rust_type(w, state, schema)?;
                writeln!(w, ",")?;
            }
        }
    }
    if let Some(any_of) = &keywords.any_of {
        for schema in &any_of.0 {
            let url = get_url(schema);
            if let Some(ResolvedJsonSchema::Object(keywords)) = state.context.schemas.get(url) {
                let ident = resolve_rust_ident(state, keywords, url);
                // writeln!(w, "\t/// {url}")?;
                writeln!(w, "\t#[serde(flatten)]")?;
                write!(w, "\tpub {ident}: Option<")?;
                resolve_rust_type(w, state, schema)?;
                write!(w, ">")?;
                writeln!(w, ",")?;
            }
        }
    }
    Ok(())
}

fn write_enum(
    w: &mut impl Write,
    state: &mut CodegenState,
    keywords: &Keywords,
    location: &Url,
) -> std::io::Result<()> {
    let ident = resolve_rust_ident(state, keywords, location);
    // writeln!(w, "/// {location}")?;
    writeln!(w, "#[derive(::serde::Deserialize, ::serde::Serialize)]")?;
    writeln!(w, "pub enum {} {{", ident)?;
    let mut wrote_true_schema = false;
    let mut wrote_false_schema = false;
    if let Some(enum_) = &keywords.enum_ {
        for value in &enum_.0 {
            match value {
                Value::String(string) => {
                    write!(w, "\t{string}")?;
                }
                _ => {}
            }
            writeln!(w, ",")?;
        }
    }
    if let Some(one_of) = &keywords.one_of {
        for schema in &one_of.0 {
            let url = get_url(schema);
            let schema = state
                .context
                .schemas
                .get(url)
                .expect("schema to be resolved");
            match schema {
                ResolvedJsonSchema::Bool(bool) => {
                    if *bool {
                        wrote_true_schema = true
                    } else {
                        wrote_false_schema = true
                    }
                }
                ResolvedJsonSchema::Object(keywords) => {
                    let ident = resolve_rust_ident(state, keywords, url);
                    write!(w, "\t{ident}({ident})")?;
                }
            }
            writeln!(w, ",")?;
        }
        if wrote_true_schema {
            writeln!(w, "\tValue(serde_json::Value),")?;
        }
        if wrote_false_schema {
            writeln!(w, "\tEmpty,")?;
        }
    }
    writeln!(w, "}}")?;
    Ok(())
}

fn write_struct(
    w: &mut impl Write,
    state: &mut CodegenState,
    keywords: &Keywords,
    location: &Url,
) -> io::Result<()> {
    let ident = resolve_rust_ident(state, keywords, location);
    writeln!(w, "/// {}", location)?;
    writeln!(w, "#[derive(::serde::Deserialize, ::serde::Serialize)]")?;
    writeln!(w, "pub struct {} {{", &ident)?;
    write_fields(w, state, keywords)?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_constant(
    w: &mut impl Write,
    state: &mut CodegenState,
    keywords: &Keywords,
    location: &Url,
) -> io::Result<()> {
    let ident = resolve_rust_ident(state, keywords, location);
    // writeln!(w, "/// {}", location)?;
    writeln!(w, "#[derive(::serde::Deserialize, ::serde::Serialize)]")?;
    writeln!(w, "pub struct {};", &ident)?;
    Ok(())
}

/// Boolean schemas are generated as serde_json::Value if true otherwise () if false.
///
/// Scalar schemas (boolean, number, string) are generated as their scalar rust equivalents.
/// A hint from format is used to determine the most likely candidate.
///
/// Object schemas are created as structs where the the properties are field names: T
/// (wrapped in option if not requried), the addiotnal properties fall in a add_props: HashMap<String, T>
/// and pattern properties are a pat_props HashMap<HashMap<String, Vec<T>>> where T is the identifier to the
/// type generated by the referenced schema.
///
/// If there are multiple types specified for a schema they are wrapped in an enum. If there is an anyOf/allOF
/// definition each schema becomes an field (optional if anyOf) with a serde(flatten) tag. If there is an oneOF definition
/// each schema will create or extend the current enum in the order they are defined.
///
/// Note that most enum variants will have serde(untagged) meaning that deserializing straight from the struct type
/// may have invalid json schema semantics for oneOf.
fn generate_rust_type(
    w: &mut impl Write,
    state: &mut CodegenState,
    context: &Context,
    location: &Url,
) -> io::Result<()> {
    let schema = context.schemas.get(location);
    match schema {
        Some(ResolvedJsonSchema::Object(keywords)) => {
            // different combination of keywords present will decide whether or not the resulting type will be a struct or enum variant.
            match keywords {
                Keywords {
                    constant: Some(_), ..
                } => {
                    write_constant(w, state, keywords, location)?;
                }
                // if a schema contains the enum keywords it must be represented as a rust enum
                Keywords { enum_: Some(_), .. } => {
                    write_enum(w, state, keywords, location)?;
                }
                // if a schema contains the oneOf keyword it must be represented as a rust enum
                Keywords {
                    one_of: Some(_), ..
                } => {
                    write_enum(w, state, keywords, location)?;
                }
                // if a schema contains multiple types it must be represented as a rust enum
                Keywords {
                    type_: Some(TypeKeyword::Multiple(_)),
                    ..
                } => {
                    write_enum(w, state, keywords, location)?;
                }
                // if a schema contains a single type and does not include the oneOf keyword it must be represented as a struct
                Keywords {
                    type_: Some(TypeKeyword::Single(ty)),
                    one_of: None,
                    ..
                } => {
                    // struct
                    match ty.as_str() {
                        "object" => {
                            write_struct(w, state, keywords, location)?;
                        }
                        _ => {
                            // no other cases
                        }
                    }
                }
                // if a schema has no type or oneOf keywords but has properties it can be represented as a struct
                Keywords {
                    props: Some(_),
                    one_of: None,
                    type_: None,
                    ..
                } => {
                    write_struct(w, state, keywords, location)?;
                }
                // if a schema has the allOf keyword but no oneOf keyword it can be represented as a struct
                Keywords {
                    all_of: Some(_),
                    one_of: None,
                    ..
                } => {
                    write_struct(w, state, keywords, location)?;
                }
                _ => {
                    writeln!(w, "// {location}")?;
                }
            }
        }
        Some(ResolvedJsonSchema::Bool(_)) => {}
        None => {}
    }
    Ok(())
}

pub fn generate_rust_code(w: &mut impl Write, context: &Context) -> io::Result<()> {
    let mut state = CodegenState::new(context);
    for (url, _) in &context.schemas {
        generate_rust_type(w, &mut state, context, url)?;
    }
    Ok(())
}
