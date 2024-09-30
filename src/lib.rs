mod endpoints;
mod errors;
mod macros;
mod models;
mod tests_utils;
mod traits;

#[cfg(test)]
static TEST_TOKEN: &'static str = "123456";

#[allow(unused)]
use serde::{Deserialize, Serialize};
#[allow(unused)]
use std::sync::{Mutex, MutexGuard, OnceLock};
pub use {
    anyhow, anyhow::Result, async_trait, futures::future::BoxFuture, lazy_static::lazy_static,
    mockito, mockito::Server, reqwest, serde_json, tokio,
};

// Clients
lazy_static! {
    pub static ref SyncClient: reqwest::blocking::Client = reqwest::blocking::Client::new();
    pub static ref AsyncClient: reqwest::Client = reqwest::Client::new();
}
