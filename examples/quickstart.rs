//! List the root of your Dropbox.
//!
//! Run with:
//!     DROPBOX_TOKEN=<your-token> cargo run --example quickstart
//!
//! Generate a token at https://www.dropbox.com/developers/apps under
//! "Generated access token". Scope required: files.metadata.read.

use rusty_dropbox_sdk::api;
use rusty_dropbox_sdk::api::Service;
use rusty_dropbox_sdk::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = match std::env::var("DROPBOX_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("set DROPBOX_TOKEN to run this example");
            return Ok(());
        }
    };

    let client = Client::new(token);
    let bearer = client.token();

    let req = api::files::list_folder::ListFolderRequest {
        access_token: &bearer,
        payload: Some(api::files::ListFolderArgs {
            path: String::new(),
            recursive: Some(false),
            include_media_info: None,
            include_deleted: None,
            include_has_explicit_shared_members: None,
            include_mounted_folders: None,
            limit: Some(50),
            shared_link: None,
            include_property_groups: None,
            include_non_downloadable_files: None,
        }),
    };

    let response = req.call().await?.expect("empty list_folder response");
    println!("found {} entries", response.payload.entries.len());
    for entry in response.payload.entries.iter().take(20) {
        println!("  {:?}", entry);
    }
    Ok(())
}
