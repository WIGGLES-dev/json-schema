use super::{CodegenState, ResolvedJsonSchema};
use crate::{context::Context, keywords::Keywords};
use url::Url;

pub(super) enum ResolveIdentError {
    Unresolvable,
    NotEnoughInfo,
    SimilarNameExists,
}

pub(super) fn try_resolve_ident<'a>(
    context: &Context,
    state: &'a mut CodegenState,
    url: &Url,
) -> Result<&'a String, ResolveIdentError> {
    if let Some(schema) = context.schemas.get(url) {
        match (url.fragment(), schema) {
            (
                Some(fragment),
                ResolvedJsonSchema::Object(Keywords {
                    title: Some(title), ..
                }),
            ) => {}
            (
                None,
                ResolvedJsonSchema::Object(Keywords {
                    title: Some(title), ..
                }),
            ) => {}
            _ => {}
        }
        state
            .idents
            .get(url)
            .map(Ok)
            .unwrap_or(Err(ResolveIdentError::Unresolvable))
    } else {
        Err(ResolveIdentError::Unresolvable)
    }
}
