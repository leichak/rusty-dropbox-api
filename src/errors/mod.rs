use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Debug;

/// Enum describing set of errors that can occur
/// Thiserror macro to derive std::error::Error trait
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    #[error("Unknown")] // display trait
    Unknown,
    #[error("Reqwest error: {0}")] // display trait
    Request(anyhow::Error),
    #[error("Parsing error: {0}")] // display trait
    Parsing(anyhow::Error),
    #[error("Dropbox error: {0}")] // display trait
    DropBox(anyhow::Error),
    /// HTTP 401. Split from `DropBox` so `Client::call` can detect expired
    /// access tokens and retry once after refreshing.
    #[error("Unauthorized: {0}")]
    Unauthorized(anyhow::Error),
}

/// Decode a Dropbox error response body into a human-readable `anyhow::Error`.
///
/// Dropbox non-2xx bodies follow the envelope
/// `{"error": <T>, "error_summary": "...", "user_message": {...}}` — this helper
/// parses that envelope, formatting both the typed `error` and the `error_summary`
/// into the returned error. If the body doesn't match the envelope shape it falls
/// back to including the raw body text. Use `serde_json::Value` as `T` for endpoints
/// that don't have a typed error enum.
/// Wrapper around a typed Dropbox error so it can ride along inside an
/// `anyhow::Error` context chain. anyhow requires attached contexts to
/// implement `Display + Debug + Send + Sync + 'static`; Dropbox error enums
/// derive only `Debug`, so we wrap them.
///
/// Callers recover the typed error via:
/// ```ignore
/// if let Some(holder) = err.downcast_ref::<TypedError<UploadError>>() {
///     match holder.get() { UploadError::PayloadTooLarge => ..., _ => ... }
/// }
/// ```
#[derive(Debug)]
pub struct TypedError<T>(pub T);

impl<T: Debug> std::fmt::Display for TypedError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: Debug + Send + Sync + 'static> std::error::Error for TypedError<T> {}

impl<T> TypedError<T> {
    pub fn get(&self) -> &T {
        &self.0
    }
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn decode_dropbox_error<T: DeserializeOwned + Debug + Send + Sync + 'static>(
    status: reqwest::StatusCode,
    body: &str,
) -> anyhow::Error {
    #[derive(Deserialize)]
    struct Envelope<T> {
        error: T,
        #[serde(default)]
        error_summary: Option<String>,
    }
    match serde_json::from_str::<Envelope<T>>(body) {
        Ok(env) => {
            let msg = format!(
                "HTTP {} — {:?} (summary: {})",
                status,
                env.error,
                env.error_summary.as_deref().unwrap_or("-"),
            );
            anyhow::Error::new(TypedError(env.error)).context(msg)
        }
        Err(_) => anyhow::anyhow!("HTTP {}: {}", status, body),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_typed_envelope() {
        let body = r#"{"error_summary": "path/not_found/.", "error": {".tag": "not_found"}}"#;
        let err = decode_dropbox_error::<serde_json::Value>(
            reqwest::StatusCode::from_u16(409).unwrap(),
            body,
        );
        let msg = err.to_string();
        assert!(msg.contains("409"));
        assert!(msg.contains("summary: path/not_found"));
    }

    #[test]
    fn downcasts_to_typed_error() {
        use crate::api::files::LookupError;
        let body = r#"{"error_summary": "path/not_found/.", "error": {".tag": "not_found"}}"#;
        let err = decode_dropbox_error::<LookupError>(
            reqwest::StatusCode::from_u16(409).unwrap(),
            body,
        );
        let typed = err
            .downcast_ref::<TypedError<LookupError>>()
            .expect("typed error attached as context");
        assert!(matches!(typed.get(), LookupError::NotFound));
    }

    #[test]
    fn falls_back_to_raw_body_on_non_envelope() {
        let body = "rate limited, try again in 30 seconds";
        let err = decode_dropbox_error::<serde_json::Value>(
            reqwest::StatusCode::from_u16(429).unwrap(),
            body,
        );
        assert!(err.to_string().contains("429"));
        assert!(err.to_string().contains("rate limited"));
    }
}
