//! Use a `Client::with_refresh(...)` so the SDK auto-refreshes the access
//! token when it expires and replays the request once on a 401.
//!
//! Run with:
//!     DROPBOX_ACCESS_TOKEN=<short-lived>   \
//!     DROPBOX_CLIENT_ID=<app-client-id>    \
//!     DROPBOX_CLIENT_SECRET=<app-secret>   \
//!     DROPBOX_REFRESH_TOKEN=<long-lived>   \
//!     cargo run --example oauth_refresh
//!
//! Acquire the refresh token via `auth::exchange_code(..., offline=true)`
//! during the standard OAuth code-grant flow.

use rusty_dropbox_sdk::api;
use rusty_dropbox_sdk::api::Service;
use rusty_dropbox_sdk::Client;
use rusty_dropbox_sdk::RefreshConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (access, client_id, client_secret, refresh_token) = match (
        std::env::var("DROPBOX_ACCESS_TOKEN"),
        std::env::var("DROPBOX_CLIENT_ID"),
        std::env::var("DROPBOX_CLIENT_SECRET"),
        std::env::var("DROPBOX_REFRESH_TOKEN"),
    ) {
        (Ok(a), Ok(b), Ok(c), Ok(d)) => (a, b, c, d),
        _ => {
            eprintln!(
                "set DROPBOX_ACCESS_TOKEN, DROPBOX_CLIENT_ID, \
                 DROPBOX_CLIENT_SECRET, DROPBOX_REFRESH_TOKEN to run"
            );
            return Ok(());
        }
    };

    let client = Client::with_refresh(
        access,
        14_400, // expires_in (seconds), from the OAuth response
        RefreshConfig {
            client_id,
            client_secret,
            refresh_token,
        },
    );

    // `client.call(...)` runs the closure with the current token; if the
    // token is expired it refreshes first, and if the server returns 401 it
    // mints a new token and replays the closure once.
    let response = client
        .call(|token| async move {
            api::files::list_folder::ListFolderRequest {
                access_token: &token,
                payload: Some(api::files::ListFolderArgs {
                    path: String::new(),
                    recursive: Some(false),
                    include_media_info: None,
                    include_deleted: None,
                    include_has_explicit_shared_members: None,
                    include_mounted_folders: None,
                    limit: Some(10),
                    shared_link: None,
                    include_property_groups: None,
                    include_non_downloadable_files: None,
                }),
            }
            .call()
            .await
        })
        .await?
        .expect("empty list_folder response");

    println!("got {} entries", response.payload.entries.len());
    Ok(())
}
