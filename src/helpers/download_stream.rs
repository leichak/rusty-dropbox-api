//! Streaming download for large files.
//!
//! The regular `DownloadRequest` buffers the entire response body into
//! `data: Vec<u8>` on the Response struct. For files above a few hundred MiB
//! that's a hard constraint — you allocate the full file in memory.
//!
//! This helper talks directly to `content.dropboxapi.com/2/files/download`
//! and returns the parsed `FileMetadata` alongside a `Stream<Item = Bytes>`
//! of body chunks, so callers can pipe to disk without buffering.

use crate::api::files::{DownloadArg, FileMetadata};
use anyhow::{Context, Result};
use bytes::Bytes;
use futures::stream::{Stream, StreamExt};

/// Open a streaming download. Returns the metadata header (fully parsed) and
/// a chunked byte stream of the file's body.
///
/// ```ignore
/// let (meta, mut stream) = download_stream(&token, "/big.zip").await?;
/// while let Some(chunk) = stream.next().await {
///     out.write_all(&chunk?).await?;
/// }
/// ```
pub async fn download_stream(
    token: &str,
    path: &str,
) -> Result<(FileMetadata, impl Stream<Item = Result<Bytes>> + Unpin)> {
    let arg = DownloadArg {
        path: path.to_string(),
        rev: None,
    };
    let arg_json = serde_json::to_string(&arg).context("serialise DownloadArg")?;

    let resp = crate::AsyncClient
        .post("https://content.dropboxapi.com/2/files/download")
        .bearer_auth(token)
        .header("Dropbox-API-Arg", arg_json)
        .send()
        .await
        .context("download request failed")?
        .error_for_status()
        .context("download returned non-2xx")?;

    let meta_header = resp
        .headers()
        .get("Dropbox-API-Result")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.replace('\n', ""))
        .context("Dropbox-API-Result header missing")?;

    let meta: FileMetadata =
        serde_json::from_str(&meta_header).context("parse Dropbox-API-Result")?;

    let stream = resp
        .bytes_stream()
        .map(|r| r.map_err(|e| anyhow::Error::new(e).context("download body stream")));

    Ok((meta, Box::pin(stream)))
}
