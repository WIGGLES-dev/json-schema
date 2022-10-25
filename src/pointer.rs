use serde::{Deserialize, Serialize};
use url::Url;

/// A JSON pointer. This can take three forms.
/// 1. A Url
/// 2. A relative or absolute file path
/// 3. A url fragment #/hello/world
/// 4. An Anchor #anchor
#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(untagged)]
pub enum JsonPointer {
    Absolute(Url),
    Relative(String),
}

impl JsonPointer {
    /// Converts a relative json pointer into an absolute pointer of some base Url.
    /// This will allocate a new Url if the underlying value is relative.
    pub fn to_absolute(&self, base: &Url) -> Result<Url, url::ParseError> {
        Ok(match self {
            JsonPointer::Absolute(url) => url.clone(),
            JsonPointer::Relative(url) => base.join(url)?,
        })
    }
    pub fn absolute(&self) -> &Url {
        match self {
            JsonPointer::Absolute(url) => url,
            _ => panic!("url is not absolute"),
        }
    }
}

impl From<Url> for JsonPointer {
    fn from(url: Url) -> Self {
        JsonPointer::Absolute(url)
    }
}

pub struct RelativeJsonPointer {
    source: String,
    traverse: usize,
}

pub struct AbsoluteJsonPointer {}

pub enum JsonPointerFragment {
    Absolute(AbsoluteJsonPointer),
    Relative(RelativeJsonPointer),
}
