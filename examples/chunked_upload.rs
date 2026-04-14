//! Stream a local file up to Dropbox via the upload_session/* endpoints,
//! which lift the 150 MiB single-request cap.
//!
//! Run with:
//!     DROPBOX_TOKEN=<your-token> cargo run --example chunked_upload -- ./local.bin /remote/path
//!
//! Scope required: files.content.write.

use rusty_dropbox_sdk::api::files::WriteMode;
use rusty_dropbox_sdk::helpers::chunked_upload::{upload_large_file, DEFAULT_CHUNK_SIZE};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = match std::env::var("DROPBOX_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("set DROPBOX_TOKEN to run this example");
            return Ok(());
        }
    };
    let mut args = std::env::args().skip(1);
    let local = args.next().unwrap_or_else(|| "./local.bin".to_string());
    let remote = args.next().unwrap_or_else(|| "/uploaded.bin".to_string());

    let file = match tokio::fs::File::open(&local).await {
        Ok(f) => f,
        Err(e) => {
            eprintln!("could not open {}: {}", local, e);
            return Ok(());
        }
    };

    let metadata =
        upload_large_file(&token, &remote, file, DEFAULT_CHUNK_SIZE, WriteMode::Add).await?;
    println!(
        "uploaded rev {} ({} bytes) to {:?}",
        metadata.rev, metadata.size, metadata.path_display
    );
    Ok(())
}
