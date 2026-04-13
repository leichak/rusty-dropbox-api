//! Chunked upload helper for files larger than Dropbox's single-request
//! `/files/upload` limit (150 MiB).
//!
//! Wraps `upload_session/{start, append_v2, finish}` so callers can feed in
//! an `AsyncRead` stream and get back the final `FileMetadata` without
//! tracking session cursors manually.

use crate::api::files::{
    upload_session_append::UploadSessionAppendRequest,
    upload_session_finish::UploadSessionFinishRequest,
    upload_session_start::UploadSessionStartRequest,
    CommitInfo, FileMetadata, UploadSessionAppendArg, UploadSessionCursor,
    UploadSessionFinishArg, UploadSessionStartArg,
};
use crate::api::Service;
use anyhow::{Context, Result};
use tokio::io::{AsyncRead, AsyncReadExt};

/// Default chunk size. 4 MiB balances round-trip count against memory use.
pub const DEFAULT_CHUNK_SIZE: usize = 4 * 1024 * 1024;

/// Upload a large file by streaming chunks through the upload_session APIs.
///
/// - `token`   — Dropbox OAuth token.
/// - `path`    — remote destination path under the user's Dropbox root.
/// - `reader`  — any `AsyncRead` stream; read until EOF.
/// - `chunk_size` — bytes per session append. Use `DEFAULT_CHUNK_SIZE`.
/// - `mode`    — Dropbox write mode string ("add", "overwrite", etc).
///
/// Returns the committed `FileMetadata`.
pub async fn upload_large_file<R: AsyncRead + Unpin>(
    token: &str,
    path: &str,
    mut reader: R,
    chunk_size: usize,
    mode: crate::api::files::WriteMode,
) -> Result<FileMetadata> {
    // Read first chunk and open the session.
    let mut first_chunk = vec![0u8; chunk_size];
    let mut first_read = 0usize;
    while first_read < chunk_size {
        let n = reader.read(&mut first_chunk[first_read..]).await?;
        if n == 0 {
            break;
        }
        first_read += n;
    }
    first_chunk.truncate(first_read);
    let eof_after_first = first_read < chunk_size;

    let start_req = UploadSessionStartRequest {
        access_token: token,
        payload: Some(UploadSessionStartArg {
            close: Some(eof_after_first),
            session_type: None,
            content_hash: None,
        }),
        data: Some(first_chunk),
    };
    let start_resp = start_req
        .call()
        .await?
        .context("upload_session/start returned empty")?;
    let session_id = start_resp.payload.session_id;

    let mut offset = first_read as u64;

    // Loop: append each chunk until we see a short read (EOF).
    if !eof_after_first {
        loop {
            let mut buf = vec![0u8; chunk_size];
            let mut read = 0usize;
            while read < chunk_size {
                let n = reader.read(&mut buf[read..]).await?;
                if n == 0 {
                    break;
                }
                read += n;
            }
            buf.truncate(read);

            if read == chunk_size {
                // Full chunk — append and continue.
                let append_req = UploadSessionAppendRequest {
                    access_token: token,
                    payload: Some(UploadSessionAppendArg {
                        cursor: UploadSessionCursor {
                            session_id: session_id.clone(),
                            offset,
                        },
                        close: Some(false),
                        content_hash: None,
                    }),
                    data: Some(buf),
                };
                let _ = append_req.call().await?;
                offset += read as u64;
            } else {
                // Short read = last chunk. Break and let finish handle it.
                let finish_req = UploadSessionFinishRequest {
                    access_token: token,
                    payload: Some(UploadSessionFinishArg {
                        cursor: UploadSessionCursor {
                            session_id: session_id.clone(),
                            offset,
                        },
                        commit: CommitInfo {
                            path: path.to_string(),
                            mode: mode.clone(),
                            autorename: true,
                            client_modified: None,
                            mute: false,
                            property_groups: None,
                            strict_conflict: None,
                        },
                        content_hash: None,
                    }),
                    data: Some(buf),
                };
                let resp = finish_req
                    .call()
                    .await?
                    .context("upload_session/finish returned empty")?;
                return Ok(resp.payload);
            }
        }
    }

    // First chunk already EOF — go straight to finish with empty body.
    let finish_req = UploadSessionFinishRequest {
        access_token: token,
        payload: Some(UploadSessionFinishArg {
            cursor: UploadSessionCursor {
                session_id,
                offset,
            },
            commit: CommitInfo {
                path: path.to_string(),
                mode,
                autorename: true,
                client_modified: None,
                mute: false,
                property_groups: None,
                strict_conflict: None,
            },
            content_hash: None,
        }),
        data: Some(Vec::new()),
    };
    let resp = finish_req
        .call()
        .await?
        .context("upload_session/finish returned empty")?;
    Ok(resp.payload)
}
