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
use crate::endpoints::{get_endpoint_url, Endpoint};
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

    // Routed through get_endpoint_url so the test-utils mock-server URL
    // rewriter (`endpoints::test_url`) intercepts when the feature is on.
    let url = get_endpoint_url(Endpoint::FilesDownloadPost)
        .2
        .unwrap_or_else(|| get_endpoint_url(Endpoint::FilesDownloadPost).0);

    let resp = crate::AsyncClient
        .post(url)
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

#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use super::download_stream;
    use crate::tests_utils::with_test_server_async;
    use futures::StreamExt;

    #[tokio::test]
    async fn streams_body_and_parses_metadata() {
        let meta_json = r#"{"name":"f.txt","id":"id:abc","client_modified":"2025-01-01T00:00:00Z","server_modified":"2025-01-01T00:00:00Z","rev":"r1","size":11,"path_lower":"/f.txt","path_display":"/f.txt","is_downloadable":true}"#;
        let body_bytes: &[u8] = b"hello world";

        with_test_server_async(|mut server| async move {
            let mock = server
                .mock("POST", "/2/files/download")
                .with_status(200)
                .with_header("Dropbox-API-Result", meta_json)
                .with_body(body_bytes)
                .create_async()
                .await;

            let (meta, mut stream) = download_stream("test", "/f.txt")
                .await
                .expect("download_stream returned error");

            assert_eq!(meta.name, "f.txt");
            assert_eq!(meta.size, 11);

            let mut got = Vec::new();
            while let Some(chunk) = stream.next().await {
                got.extend_from_slice(&chunk.expect("chunk error"));
            }
            assert_eq!(got, body_bytes);
            mock.assert();
        })
        .await;
    }
}
