use crate::{
    pointer::JsonPointer,
    schema::{JsonSchema, ResolvedJsonSchema},
};
use reqwest::Client;
use std::{
    collections::HashMap,
    fmt::Write,
    fs::File,
    path::{Path, PathBuf},
};
use url::Url;

#[derive(Debug)]
pub enum CompileError {
    UnsupportedScheme,
    UnsupportedExtension,
    InvalidPath,
    InvalidUrl,
    NetworkError,
    ParseError,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for CompileError {}

/// A reference to a json schema document, there are two variants.

/// The first Ref(Url) resolves to a single json schema document.
///
/// The secnd Mod(Url) resolves to a set of json schema documents, this
/// is useful when dealing with the local file system. With remote schemas
/// the http response should return a map of schemas.
pub enum Reference {
    Ref(Url),
    Mod(Url),
}

pub struct Compiler<'a> {
    json_pointer: String,
    location: &'a mut Url,
    refs: &'a mut Vec<Reference>,
    context: &'a mut Context,
}

impl<'a> Compiler<'a> {
    pub fn compile_rel_key(&mut self, f: impl FnOnce(&mut String), schema: &mut JsonSchema) {
        // store len of string to truncate value back to current location
        let truncate = self.json_pointer.len();
        (f)(&mut self.json_pointer);
        self.location.set_fragment(Some(&self.json_pointer));
        self.compile(schema);
        self.json_pointer.truncate(truncate);
    }
    pub fn compile(&mut self, schema: &mut JsonSchema) {
        match {
            match schema {
                JsonSchema::Ref { ref_ } => ref_.to_absolute(self.location),
                JsonSchema::Mod { mod_ } => mod_.to_absolute(self.location),
                _ => Ok(self.location.clone()),
            }
        } {
            Ok(schema_url) => {
                if self.context.schemas.contains_key(&schema_url) {
                    *schema = JsonSchema::Resolved(schema_url);
                } else {
                    let mut schema =
                        std::mem::replace(schema, JsonSchema::Resolved(schema_url.clone()));
                    match &mut schema {
                        // if the nested schema is an object we do not need to extract, simply ask all nested keywords to resolve their values.
                        JsonSchema::Object(keywords) => {
                            keywords.compile(self);
                        }
                        JsonSchema::Ref { .. } => {
                            self.refs.push(Reference::Ref(schema_url.clone()));
                        }
                        JsonSchema::Mod { .. } => {
                            self.refs.push(Reference::Mod(schema_url.clone()));
                        }
                        _ => {}
                    }
                    match schema {
                        JsonSchema::Bool(bool) => {
                            self.context
                                .schemas
                                .insert(schema_url, ResolvedJsonSchema::Bool(bool));
                        }
                        JsonSchema::Object(keywords) => {
                            self.context
                                .schemas
                                .insert(schema_url, ResolvedJsonSchema::Object(keywords));
                        }
                        // will be handled by resolved collected urls
                        JsonSchema::Resolved(_) => {}
                        // should never occur because the ref should be resolved
                        JsonSchema::Ref { .. } => {}
                        // should never occur because the mod should b resolved
                        JsonSchema::Mod { .. } => {}
                    };
                }
            }
            Err(_) => {}
        }
    }
}

pub struct Context {
    client: Client,
    pub schemas: HashMap<Url, ResolvedJsonSchema>,
}

impl Context {
    pub fn clear(&mut self) {
        self.schemas.clear();
    }
    pub fn schema(&self, url: &Url) -> Option<&ResolvedJsonSchema> {
        self.schemas.get(url)
    }
    /// If you know the locations of your schemas ahead of time you can prefetch them in order
    /// to speed compilation. This avoid having to wait for one schema to resolve before the
    /// next request can be made.
    pub async fn prefetch(prefetch: impl IntoIterator<Item = Reference>) -> Self {
        let mut context = Self::new();
        context.resolve_refs(prefetch).await;
        context
    }
    pub fn new() -> Self {
        let client = Client::new();
        let schemas = HashMap::new();
        Self { schemas, client }
    }

    /// Resolve a series of references in parallel.
    pub async fn resolve_refs(&mut self, refs: impl IntoIterator<Item = Reference>) {
        for ref_ in refs {
            match ref_ {
                Reference::Mod(url) => {}
                Reference::Ref(url) => {
                    match self.compile_url(url).await {
                        Ok(_) => {}
                        Err(_) => {}
                    };
                }
            }
        }
    }
    /// Compile a schema object directly. All related schemas will be recursiveley acquired and
    /// available in the schemas map at their url.
    pub async fn compile_schema(&mut self, schema: &mut JsonSchema, mut location: Url) {
        let mut refs = vec![];
        Compiler {
            json_pointer: String::new(),
            location: &mut location,
            refs: &mut refs,
            context: self,
        }
        .compile(schema);
        // resolve the pending urls asynchronously
        self.resolve_refs(refs).await;
    }

    pub async fn resolve_file(&mut self, location: &Url) -> Result<JsonSchema, CompileError> {
        let path = location
            .to_file_path()
            .map_err(|_| CompileError::InvalidPath)?;
        let mut file = File::open(&path).map_err(|_| CompileError::InvalidPath)?;
        let schema: JsonSchema =
            if let Some(pointer) = location.fragment() {
                let mut value: serde_json::Value =
                    match path.extension().and_then(|os_str| os_str.to_str()) {
                        Some("yml" | "yaml") => serde_yaml::from_reader(&mut file)
                            .map_err(|_| CompileError::ParseError)?,
                        Some("json") => serde_json::from_reader(&mut file)
                            .map_err(|_| CompileError::ParseError)?,
                        _ => return Err(CompileError::UnsupportedExtension),
                    };
                if let Some(value) = value.pointer_mut(pointer).map(|v| v.take()) {
                    serde_json::from_value(value).map_err(|_| CompileError::ParseError)?
                } else {
                    return Err(CompileError::ParseError);
                }
            } else {
                match path.extension().and_then(|os_str| os_str.to_str()) {
                    Some("yml" | "yaml") => {
                        serde_yaml::from_reader(&mut file).map_err(|_| CompileError::ParseError)?
                    }
                    Some("json") => {
                        serde_json::from_reader(&mut file).map_err(|_| CompileError::ParseError)?
                    }
                    _ => return Err(CompileError::UnsupportedExtension),
                }
            };
        Ok(schema)
    }

    pub async fn resolve_http(&mut self, location: &Url) -> Result<JsonSchema, CompileError> {
        let res = self
            .client
            .get(location.clone())
            .send()
            .await
            .map_err(|_| CompileError::NetworkError)?;
        let schema: JsonSchema = res.json().await.map_err(|_| CompileError::ParseError)?;
        Ok(schema)
    }

    /// compile all valid schemas recursively in a directory.
    pub async fn compile_dir(&mut self, path: impl AsRef<Path>) -> Result<(), CompileError> {
        let mut dirs: Vec<PathBuf> = vec![PathBuf::from(path.as_ref())];
        while let Some(path) = dirs.pop() {
            for dirent in std::fs::read_dir(path).map_err(|_| CompileError::InvalidPath)? {
                match dirent {
                    Ok(dirent) => {
                        let path = dirent.path();
                        if path.is_file() {
                            let path = std::fs::canonicalize(dirent.path())
                                .map_err(|_| CompileError::InvalidPath)?;
                            let url =
                                Url::from_file_path(path).map_err(|_| CompileError::InvalidPath)?;
                            self.compile_url(url).await?;
                        } else if path.is_dir() {
                            dirs.push(path);
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    /// Compile schema at url. After processing all schemas will be available at their resolved url
    /// in the context object.
    #[async_recursion::async_recursion]
    pub async fn compile_url(
        &mut self,
        location: impl TryInto<Url> + Send + 'static,
    ) -> Result<JsonSchema, CompileError> {
        let location: Url = location.try_into().map_err(|_| CompileError::InvalidUrl)?;
        if self.schemas.contains_key(&location) {
            Ok(JsonSchema::Resolved(location))
        } else {
            let mut schema = match location.scheme() {
                "http" | "https" => self.resolve_http(&location).await,
                "file" => self.resolve_file(&location).await,
                _ => Err(CompileError::UnsupportedScheme),
            }?;
            self.compile_schema(&mut schema, location).await;
            Ok(schema)
        }
    }
}
