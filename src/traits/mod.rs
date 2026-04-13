use anyhow::Result;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

/// Trait implemented by every `*Request` type.
///
/// The `call()` method returns a boxed future directly — there is no outer
/// `Result` wrapper because constructing the future never fails today.
/// Callers use:
///
/// ```ignore
/// let result = request.call().await?;
/// let result = request.call_sync()?;
/// ```
#[allow(unused)]
pub trait Service<O: Sized> {
    fn call_sync(&self) -> Result<Option<O>>;
    fn call(&self) -> BoxFuture<'static, Result<Option<O>>>;
}

pub trait Utils<'a> {
    type T: Serialize + Deserialize<'a>;
    fn payload(&self) -> Option<&Self::T>;

    /// Binary request body for content-endpoints (upload_session/*, upload).
    /// Default is None — only overridden by Request types that actually carry
    /// file bytes.
    fn content_body(&self) -> Option<&[u8]> {
        None
    }
}
