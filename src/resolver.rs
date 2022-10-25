use crate::{context::Context, schema::JsonSchema};

pub enum Error {
    InvalidRelativePath,
    InvalidFragment,
    InvalidUrl(url::ParseError),
}

pub type Result = std::result::Result<JsonSchema, Error>;

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::InvalidUrl(err)
    }
}

pub async fn default_resolver(cx: Context) -> Result {
    Err(Error::InvalidFragment)
}
