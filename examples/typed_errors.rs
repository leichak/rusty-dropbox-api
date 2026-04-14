//! Recover typed Dropbox errors instead of inspecting error strings.
//!
//! Calls `get_metadata` on a path that doesn't exist, then downcasts the
//! returned anyhow::Error to `TypedError<LookupError>` so we can match on
//! `LookupError::NotFound`.
//!
//! Run with:
//!     DROPBOX_TOKEN=<your-token> cargo run --example typed_errors

use rusty_dropbox_sdk::api;
use rusty_dropbox_sdk::api::files::LookupError;
use rusty_dropbox_sdk::api::Service;
use rusty_dropbox_sdk::TypedError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = match std::env::var("DROPBOX_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("set DROPBOX_TOKEN to run this example");
            return Ok(());
        }
    };

    let req = api::files::get_metadata::GetMetadataRequest {
        access_token: &token,
        payload: Some(api::files::GetMetadataArgs {
            path: "/this/path/does/not/exist".to_string(),
            include_media_info: None,
            include_deleted: None,
            include_has_explicit_shared_members: None,
            include_property_groups: None,
        }),
    };

    match req.call().await {
        Ok(_) => println!("metadata fetched (the path actually existed!)"),
        Err(e) => {
            if let Some(holder) = e.downcast_ref::<TypedError<LookupError>>() {
                match holder.get() {
                    LookupError::NotFound => println!("expected: path not found"),
                    other => println!("other lookup error: {:?}", other),
                }
            } else {
                eprintln!("non-typed error: {e}");
            }
        }
    }
    Ok(())
}
