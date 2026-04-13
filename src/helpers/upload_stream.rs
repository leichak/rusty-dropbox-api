//! Single-call streaming upload to `/files/upload`.
//!
//! `chunked_upload::upload_large_file` already streams via the
//! upload_session/* triple — that's the correct path for files over 150 MiB.
//! This helper covers the *under*-150 MiB case where you have a reader and
//! don't want to buffer the whole thing into a `Vec<u8>` to hand to
//! `UploadRequest.data`.

use crate::api::files::{FileMetadata, UploadArg, WriteMode};
use crate::endpoints::{get_endpoint_url, Endpoint};
use anyhow::{Context, Result};
use bytes::Bytes;
use futures::stream;
use tokio::io::{AsyncRead, AsyncReadExt};

const CHUNK_SIZE: usize = 64 * 1024;

/// Upload a file in a single `/files/upload` POST, streaming the body from
/// any `AsyncRead`. The reader is consumed; Dropbox enforces a 150 MiB cap
/// on this endpoint — for larger files use
/// [`crate::helpers::chunked_upload::upload_large_file`].
pub async fn upload_stream<R>(
    token: &str,
    path: &str,
    reader: R,
    mode: WriteMode,
) -> Result<FileMetadata>
where
    R: AsyncRead + Send + Sync + Unpin + 'static,
{
    let arg = UploadArg {
        path: path.to_string(),
        mode,
        autorename: Some(true),
        client_modified: None,
        mute: Some(false),
        property_groups: None,
        strict_conflict: None,
        content_hash: None,
    };
    let arg_json = serde_json::to_string(&arg).context("serialise UploadArg")?;

    // Adapter: AsyncRead -> Stream<Item = Result<Bytes, io::Error>>. No
    // tokio-util dep needed; each poll reads one chunk into a fresh Vec.
    let body_stream = stream::unfold(reader, |mut reader| async move {
        let mut buf = vec![0u8; CHUNK_SIZE];
        match reader.read(&mut buf).await {
            Ok(0) => None,
            Ok(n) => {
                buf.truncate(n);
                Some((
                    Ok::<Bytes, std::io::Error>(Bytes::from(buf)),
                    reader,
                ))
            }
            Err(e) => Some((Err(e), reader)),
        }
    });
    let body = reqwest::Body::wrap_stream(body_stream);

    let url = get_endpoint_url(Endpoint::FilesUploadPost)
        .2
        .unwrap_or_else(|| get_endpoint_url(Endpoint::FilesUploadPost).0);

    let resp = crate::AsyncClient
        .post(url)
        .bearer_auth(token)
        .header("Content-Type", "application/octet-stream")
        .header("Dropbox-API-Arg", arg_json)
        .body(body)
        .send()
        .await
        .context("upload request failed")?
        .error_for_status()
        .context("upload returned non-2xx")?;

    let meta: FileMetadata = resp.json().await.context("parse upload response")?;
    Ok(meta)
}

#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use super::upload_stream;
    use crate::api::files::WriteMode;
    use crate::tests_utils::get_mut_or_init_async;
    use std::io::Cursor;

    #[tokio::test]
    async fn streams_payload_and_parses_metadata() {
        let meta_json =
            r#"{"name":"f.txt","id":"id:abc","client_modified":"2025-01-01T00:00:00Z","server_modified":"2025-01-01T00:00:00Z","rev":"r1","size":5,"path_lower":"/f.txt","path_display":"/f.txt","is_downloadable":true}"#;

        let mock;
        {
            let mut server = get_mut_or_init_async().await;
            mock = server
                .mock("POST", "/2/files/upload")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(meta_json)
                .create_async()
                .await;
        }

        let reader = Cursor::new(b"hello".to_vec());
        let meta = upload_stream("test", "/f.txt", reader, WriteMode::Add)
            .await
            .expect("upload_stream returned error");
        assert_eq!(meta.name, "f.txt");
        assert_eq!(meta.size, 5);
        mock.assert();
    }
}
