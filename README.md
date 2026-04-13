
# Dropbox for Rust

**Unofficial Dropbox API SDK for Rust**

This crate provides a simple and idiomatic Rust interface to interact with the Dropbox API. It supports common Dropbox operations such as file uploads, downloads, and handling OAuth2 tokens. This SDK is asynchronous and integrates well with Rust's async ecosystem, utilizing `tokio`.

## Features

- Partial support for Dropbox API v2, planned full support.
- Token authentication.
- Async and sync API calls.

## Supported Endpoints

The following Dropbox API categories have support in the SDK:

- `account`
- `auth`
- `check`
- `contacts`
- `file_properties`
- `file_requests`
- `files` — all v2 endpoints (copy, move, delete, download, upload,
  upload_session/\*, list_folder, search, tags, lock_file, paper, etc.)

Planned:
- `sharing`, `users`

Full support for all Dropbox API endpoints is coming soon!

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rusty_dropbox_sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Usage

Here's a basic example showing how to revoke an OAuth2 token.

### Sync Example

```rust
use dropbox_api::api;
use dropbox_api::api::Service;

fn main() {
    let request = api::auth::token_revoke::TokenRevokeRequest {
        access_token: "your_access_token",
        payload: None,
    };

    match Service::call_sync(&request) {
        Ok(Some(result)) => println!("Token revoked: {:?}", result),
        _ => println!("Failed to revoke token or connection not present"),
    }
}
```

### Async Example

```rust
use dropbox_api::api;
use tokio;

#[tokio::main]
async fn main() {
    let request = api::auth::token_revoke::TokenRevokeRequest {
        access_token: "your_access_token",
        payload: None,
    };

    match request.call().await {
        Ok(result) => println!("Token revoked: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

### Listing a folder

```rust
use rusty_dropbox_sdk::api;
use rusty_dropbox_sdk::api::Service;

#[tokio::main]
async fn main() {
    let request = api::files::list_folder::ListFolderRequest {
        access_token: "your_access_token",
        payload: Some(api::files::ListFolderArgs {
            path: "".to_string(),      // "" = root in Dropbox
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

    match request.call().await {
        Ok(Some(result)) => {
            for entry in result.payload.entries {
                println!("{:?}", entry);
            }
        }
        Ok(None) => println!("Empty response"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

### Uploading a large file

```rust
use rusty_dropbox_sdk::helpers::chunked_upload;
use tokio::fs::File;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = File::open("./video.mp4").await?;
    let metadata = chunked_upload::upload_large_file(
        "your_access_token",
        "/videos/clip.mp4",
        file,
        chunked_upload::DEFAULT_CHUNK_SIZE,
        "add",
    )
    .await?;
    println!("Uploaded rev {} ({} bytes)", metadata.rev, metadata.size);
    Ok(())
}
```

### Recovering typed errors

```rust
use rusty_dropbox_sdk::{api, TypedError};
use rusty_dropbox_sdk::api::files::LookupError;
use rusty_dropbox_sdk::api::Service;

# async fn run() -> anyhow::Result<()> {
let req = api::files::get_metadata::GetMetadataRequest {
    access_token: "your_access_token",
    payload: Some(api::files::GetMetadataArgs {
        path: "/does-not-exist".to_string(),
        include_media_info: None,
        include_deleted: None,
        include_has_explicit_shared_members: None,
        include_property_groups: None,
    }),
};

match req.call().await {
    Ok(_) => println!("metadata fetched"),
    Err(e) => {
        if let Some(holder) = e.downcast_ref::<TypedError<LookupError>>() {
            match holder.get() {
                LookupError::NotFound => println!("that path isn't there"),
                _ => println!("other lookup error: {:?}", holder.get()),
            }
        } else {
            eprintln!("non-typed error: {e}");
        }
    }
}
# Ok(())
# }
```

### Advanced Usage

Here’s an advanced example of creating a file request using Dropbox API:

```rust
use dropbox_api::api::file_requests::{CreateFileRequestArgs, FileRequestDeadline};
use dropbox_api::api::Service;
use chrono::DateTime;

#[tokio::main]
async fn main() {
    let request = api::file_requests::create::CreateFileRequest {
        access_token: "your_access_token",
        payload: Some(CreateFileRequestArgs {
            title: "File Request".to_string(),
            destination: "/path/to/destination".to_string(),
            deadline: Some(FileRequestDeadline {
                deadline: DateTime::from_timestamp_millis(1690000000000).unwrap(),
                allow_late_uploads: None,
            }),
            open: false,
            description: Some("A request for a file.".to_string()),
            video_project_id: None,
        }),
    };

    match request.call().await {
        Ok(result) => println!("File request created: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

## Running Tests

To run integration tests, use the following command:

```sh
cargo test --test '*'
```

To run local tests, use following commands:

```sh
cargo test --features "test-utils"
```

## Contributing

This is an unofficial release of the Dropbox API SDK for Rust. Contributions and issues are welcome. Please follow the standard [GitHub flow](https://guides.github.com/introduction/flow/) for contributions.

## License

This project is licensed under the GNU General Public License v3.0
