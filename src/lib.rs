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
//! let req = api::files::list_folder::ListFolderRequest {
//!     access_token: &token,
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
//! let result = req.call()?.await?;
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
mod endpoints;
mod errors;
mod macros;
mod models;
mod tests_utils;
mod traits;

#[cfg(test)]
static TEST_AUTH_TOKEN: &'static str = "12345";

#[allow(unused)]
use serde::{Deserialize, Serialize};
#[allow(unused)]
use std::sync::{Mutex, MutexGuard, OnceLock};
use {anyhow, futures::future::BoxFuture, lazy_static::lazy_static};

// Clients
lazy_static! {
    static ref SyncClient: reqwest::blocking::Client = reqwest::blocking::Client::new();
    static ref AsyncClient: reqwest::Client = reqwest::Client::new();
}
