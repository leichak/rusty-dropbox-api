//! Live integration tests against real Dropbox.
//!
//! These tests run against https://api.dropboxapi.com using a real OAuth token.
//! They are intentionally env-gated so `cargo test` on a fresh checkout does
//! nothing — only contributors with credentials execute them.
//!
//! # Running
//!
//! ```sh
//! export DROPBOX_TEST_TOKEN="<a-personal-access-token-with-files.metadata.read>"
//! cargo test --test live_dropbox -- --nocapture
//! ```
//!
//! A token can be generated at https://www.dropbox.com/developers/apps under
//! "Generated access token". Scope required for the current tests:
//!   - account_info.read
//!   - files.metadata.read
//!
//! # Scope of coverage
//!
//! Today this exercises only read-only endpoints that don't depend on the
//! deferred binary-body API (Jobs 5b and 8b). Once those land we can extend
//! with a golden-path upload -> download -> metadata -> delete sequence under
//! a scratch folder `/rusty_dropbox_sdk_integration_<uuid>/`.

use rusty_dropbox_sdk::api;
use rusty_dropbox_sdk::api::Service;

/// Returns the token from DROPBOX_TEST_TOKEN, or `None` if unset. Every live
/// test opens with `let token = match live_token() { Some(t) => t, None => return };`
/// so the suite silently no-ops when the env var is missing.
fn live_token() -> Option<String> {
    std::env::var("DROPBOX_TEST_TOKEN").ok()
}

#[tokio::test]
async fn list_root_folder() {
    let token = match live_token() {
        Some(t) => t,
        None => {
            eprintln!("DROPBOX_TEST_TOKEN not set — skipping");
            return;
        }
    };

    let request = api::files::list_folder::ListFolderRequest {
        access_token: &token,
        payload: Some(api::files::ListFolderArgs {
            path: "".to_string(),
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
    };

    let result = request.call().await.expect("list_folder returned Err");

    let result = result.expect("empty response");
    println!(
        "list_folder returned {} entries, cursor={}, has_more={}",
        result.payload.entries.len(),
        result.payload.cursor,
        result.payload.has_more,
    );
}

#[tokio::test]
async fn list_root_collect_all() {
    let token = match live_token() {
        Some(t) => t,
        None => {
            eprintln!("DROPBOX_TEST_TOKEN not set — skipping");
            return;
        }
    };

    let request = api::files::list_folder::ListFolderRequest {
        access_token: &token,
        payload: Some(api::files::ListFolderArgs {
            path: "".to_string(),
            recursive: Some(false),
            include_media_info: None,
            include_deleted: None,
            include_has_explicit_shared_members: None,
            include_mounted_folders: None,
            limit: Some(2), // force pagination on any non-empty root
            shared_link: None,
            include_property_groups: None,
            include_non_downloadable_files: None,
        }),
    };

    let entries = request.collect_all().await.expect("collect_all failed");
    println!("collect_all walked {} total entries", entries.len());
}

#[tokio::test]
async fn list_root_sync() {
    let token = match live_token() {
        Some(t) => t,
        None => {
            eprintln!("DROPBOX_TEST_TOKEN not set — skipping");
            return;
        }
    };

    let request = api::files::list_folder::ListFolderRequest {
        access_token: &token,
        payload: Some(api::files::ListFolderArgs {
            path: "".to_string(),
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
    };

    let result = request
        .call_sync()
        .expect("sync list_folder failed")
        .expect("empty response");
    println!(
        "sync list_folder returned {} entries",
        result.payload.entries.len(),
    );
}
