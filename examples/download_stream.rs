//! Stream a Dropbox file straight to disk without buffering it in memory.
//!
//! Run with:
//!     DROPBOX_TOKEN=<your-token> cargo run --example download_stream -- /remote/path local.bin
//!
//! Scope required: files.content.read.

use futures::StreamExt;
use rusty_dropbox_sdk::helpers::download_stream::download_stream;
use tokio::io::AsyncWriteExt;

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
    let remote = args.next().unwrap_or_else(|| "/example.bin".to_string());
    let local = args
        .next()
        .unwrap_or_else(|| "./downloaded.bin".to_string());

    let (meta, mut stream) = download_stream(&token, &remote).await?;
    println!(
        "downloading {} ({} bytes) -> {}",
        meta.name, meta.size, local
    );

    let mut out = tokio::fs::File::create(&local).await?;
    let mut total = 0u64;
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        out.write_all(&bytes).await?;
        total += bytes.len() as u64;
    }
    out.flush().await?;
    println!("wrote {} bytes", total);
    Ok(())
}
