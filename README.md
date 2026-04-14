# rusty_dropbox_sdk

[![crates.io](https://img.shields.io/crates/v/rusty_dropbox_sdk.svg)](https://crates.io/crates/rusty_dropbox_sdk)
[![docs.rs](https://docs.rs/rusty_dropbox_sdk/badge.svg)](https://docs.rs/rusty_dropbox_sdk)
[![license](https://img.shields.io/crates/l/rusty_dropbox_sdk.svg)](./LICENSE)

Unofficial Rust SDK for the Dropbox HTTP v2 API. Async-first on `reqwest`,
batteries-included: built-in OAuth refresh, retries on 429 / 5xx, and
streaming up- and downloads. Both async and sync are available on every
endpoint.

## Status

`0.8.x` — usable in real workloads (246 mocked tests + a live integration
suite); the public surface may still see breaking changes before `1.0`. Not
affiliated with Dropbox. Another community SDK lives at
[`dropbox/dropbox-sdk-rust`](https://github.com/dropbox/dropbox-sdk-rust).

## Why this crate

- **Zero-config HTTP** — `Client::new(token)` and you're going. No
  `HttpClient` trait to implement.
- **OAuth refresh built in** — `Client::with_refresh(...)` plus
  `client.call(|token| ...)` auto-refreshes when expired and replays once
  on a 401.
- **Automatic retries** on 429 and 5xx with exponential backoff, baked into
  the request macro.
- **Streaming helpers** — `download_stream` returns a
  `futures::Stream<Item = Bytes>`; `chunked_upload::upload_large_file` lifts
  the 150 MiB single-request cap.
- **Sync and async on every Request** — call `.call().await` or
  `.call_sync()` from the same struct. No feature toggling.
- **Typed per-endpoint errors** — downcast `anyhow::Error` to
  `TypedError<E>` (e.g. `TypedError<LookupError>`) and match the variant.
- **Stone-spec naming** preserved across the 11 namespaces — types map 1:1
  to the Dropbox IDL.

## Install

```toml
[dependencies]
rusty_dropbox_sdk = "0.8"
tokio = { version = "1", features = ["full"] }
```

MSRV: `1.75`.

## Quick start

```rust,no_run
use rusty_dropbox_sdk::api;
use rusty_dropbox_sdk::api::Service;
use rusty_dropbox_sdk::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(std::env::var("DROPBOX_TOKEN")?);
    let bearer = client.token();

    let req = api::files::list_folder::ListFolderRequest {
        access_token: &bearer,
        payload: Some(api::files::ListFolderArgs {
            path: String::new(),
            recursive: Some(false),
            limit: Some(50),
            include_media_info: None,
            include_deleted: None,
            include_has_explicit_shared_members: None,
            include_mounted_folders: None,
            shared_link: None,
            include_property_groups: None,
            include_non_downloadable_files: None,
        }),
    };
    let response = req.call().await?.expect("empty response");
    for entry in response.payload.entries {
        println!("{:?}", entry);
    }
    Ok(())
}
```

## Authentication

Two ways to construct a client:

```rust,no_run
use rusty_dropbox_sdk::{Client, RefreshConfig};

// 1. Bring your own short-lived access token. No refresh — once it expires
//    every request returns 401 until you build a new client.
let _client = Client::new("short-lived-access-token");

// 2. Auto-refreshing client. Pass the OAuth app's client_id, client_secret,
//    and a long-lived refresh token (acquired via auth::exchange_code with
//    offline=true). client.call(...) will refresh + replay on 401.
let _client = Client::with_refresh(
    "current-access-token",
    14_400, // expires_in (seconds), from the OAuth response
    RefreshConfig {
        client_id:     std::env::var("DROPBOX_CLIENT_ID").unwrap(),
        client_secret: std::env::var("DROPBOX_CLIENT_SECRET").unwrap(),
        refresh_token: std::env::var("DROPBOX_REFRESH_TOKEN").unwrap(),
    },
);
```

## Coverage

Implemented and tested namespaces:

- `account` — set profile photo
- `auth` — OAuth code exchange, refresh, token revoke
- `check` — `app` and `user` health probes
- `contacts` — manual contacts
- `file_properties` — properties + templates (Stone-IDL naming)
- `file_requests` — create, get, list, count, delete
- `files` — all v2 endpoints (copy, move, delete, download, upload,
  upload_session/\*, list_folder, search, tags, lock_file, paper, etc.)
- `openid` — `userinfo`
- `sharing` — folders, file members, shared links, invitees
- `users` — account, current account, space usage, features

Out of scope today: team-admin endpoints (`team*` namespaces).

## Examples

Runnable end-to-end programs live under [`examples/`](./examples). Each
reads credentials from environment variables and silently skips when they
aren't set, so feel free to run any of them with no setup.

```sh
DROPBOX_TOKEN=<your-token> cargo run --example quickstart
DROPBOX_TOKEN=<your-token> cargo run --example download_stream -- /remote/big.zip ./big.zip
DROPBOX_TOKEN=<your-token> cargo run --example chunked_upload  -- ./local.bin /remote/path
DROPBOX_TOKEN=<your-token> cargo run --example typed_errors

DROPBOX_ACCESS_TOKEN=...  DROPBOX_CLIENT_ID=...  DROPBOX_CLIENT_SECRET=...  \
DROPBOX_REFRESH_TOKEN=... cargo run --example oauth_refresh
```

A few inline recipes for the most-asked questions:

<details>
<summary>Streaming a large download to disk</summary>

```rust,no_run
use rusty_dropbox_sdk::helpers::download_stream::download_stream;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (meta, mut stream) = download_stream("your_token", "/big.zip").await?;
    println!("downloading {} ({} bytes)", meta.name, meta.size);
    let mut out = tokio::fs::File::create("./big.zip").await?;
    while let Some(chunk) = stream.next().await {
        out.write_all(&chunk?).await?;
    }
    Ok(())
}
```
</details>

<details>
<summary>Uploading a large file via upload_session/*</summary>

```rust,no_run
use rusty_dropbox_sdk::api::files::WriteMode;
use rusty_dropbox_sdk::helpers::chunked_upload::{upload_large_file, DEFAULT_CHUNK_SIZE};
use tokio::fs::File;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = File::open("./video.mp4").await?;
    let metadata = upload_large_file(
        "your_token",
        "/videos/clip.mp4",
        file,
        DEFAULT_CHUNK_SIZE,
        WriteMode::Add,
    )
    .await?;
    println!("uploaded rev {} ({} bytes)", metadata.rev, metadata.size);
    Ok(())
}
```
</details>

<details>
<summary>Recovering typed errors</summary>

```rust,no_run
use rusty_dropbox_sdk::{api, TypedError};
use rusty_dropbox_sdk::api::files::LookupError;
use rusty_dropbox_sdk::api::Service;

# async fn run() -> anyhow::Result<()> {
let req = api::files::get_metadata::GetMetadataRequest {
    access_token: "your_token",
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
                other => println!("other lookup error: {:?}", other),
            }
        } else {
            eprintln!("non-typed error: {e}");
        }
    }
}
# Ok(())
# }
```
</details>

<details>
<summary>Sync calls</summary>

Every request implements both `call()` and `call_sync()`:

```rust,no_run
use rusty_dropbox_sdk::api;
use rusty_dropbox_sdk::api::Service;

fn main() -> anyhow::Result<()> {
    let req = api::auth::token_revoke::TokenRevokeRequest {
        access_token: "your_token",
        payload: None,
    };
    let _ = Service::call_sync(&req)?;
    Ok(())
}
```
</details>

## Feature flags

- `test-utils` — pulls in `mockito` and exposes the per-test ephemeral
  mock-server helpers used by the SDK's own test suite. Not needed for
  normal use; default builds skip it entirely.

## Running tests

```sh
cargo test --features test-utils --lib
```

Integration tests against the live Dropbox API live in
[`tests/live_dropbox.rs`](./tests/live_dropbox.rs) and are env-gated — they
silently no-op without `DROPBOX_TEST_TOKEN`. Run them explicitly with:

```sh
DROPBOX_TEST_TOKEN=<your-token> cargo test --test live_dropbox -- --nocapture
```

## Contributing

Issues and PRs welcome. Please follow the standard
[GitHub flow](https://guides.github.com/introduction/flow/).

## License

GPL-3.0-only. See [LICENSE](./LICENSE).
