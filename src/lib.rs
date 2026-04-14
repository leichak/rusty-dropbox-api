//! Unofficial Dropbox HTTP API v2 SDK for Rust.
//!
//! # Quick start
//!
//! ```no_run
//! use rusty_dropbox_sdk::api;
//! use rusty_dropbox_sdk::api::Service;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let token = std::env::var("DROPBOX_TOKEN")?;
//! let client = rusty_dropbox_sdk::Client::new(token);
//! let bearer = client.token();
//! let req = api::files::list_folder::ListFolderRequest {
//!     access_token: &bearer,
//!     payload: Some(api::files::ListFolderArgs {
//!         path: String::new(),
//!         recursive: Some(false),
//!         include_media_info: None,
//!         include_deleted: None,
//!         include_has_explicit_shared_members: None,
//!         include_mounted_folders: None,
//!         limit: Some(50),
//!         shared_link: None,
//!         include_property_groups: None,
//!         include_non_downloadable_files: None,
//!     }),
//! };
//! let result = req.call().await?;
//! println!("{:?}", result);
//! # Ok(())
//! # }
//! ```
//!
//! # Organisation
//!
//! - [`api`] — request/response types grouped by Dropbox namespace
//!   (`account`, `auth`, `check`, `contacts`, `file_properties`,
//!   `file_requests`, `files`). Each endpoint has its own submodule
//!   with a `*Request` struct you build and call.
//! - Every request implements the [`Service`](api::Service) trait which
//!   exposes both `call()` (async) and `call_sync()` (blocking) methods.
//!
//! # Reference
//!
//! Dropbox HTTP API docs: <https://www.dropbox.com/developers/documentation/http/documentation>

pub mod api;
pub mod auth;
mod client;
mod endpoints;
mod errors;
pub mod helpers;
mod macros;
mod models;
mod tests_utils;
mod traits;

pub use client::{Client, RefreshConfig};
pub use errors::TypedError;

/// Ergonomic re-exports. `use rusty_dropbox_sdk::prelude::*;` brings in the
/// `Service` trait (so `request.call().await?` resolves), the `Client`
/// token holder, and the `files` namespace.
pub mod prelude {
    pub use crate::api::files;
    pub use crate::api::Service;
    pub use crate::Client;
}

#[cfg(test)]
static TEST_AUTH_TOKEN: &str = "12345";

#[allow(unused)]
use serde::{Deserialize, Serialize};
#[allow(unused)]
use std::sync::{Mutex, MutexGuard, OnceLock};
// `use anyhow;` is intentional: it exposes `crate::anyhow` to dozens of
// `src/models/**/*.rs` files which import `Result` via `crate::anyhow::Result`.
#[allow(clippy::single_component_path_imports)]
use {anyhow, lazy_static::lazy_static};

/// User-Agent advertised on every request. Lets Dropbox support trace traffic
/// from this SDK and lets you bump the version when filing issues against them.
pub const USER_AGENT: &str = concat!("rusty_dropbox_sdk/", env!("CARGO_PKG_VERSION"));

// Clients
lazy_static! {
    static ref SyncClient: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("sync client");
    static ref AsyncClient: reqwest::Client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("async client");
}
