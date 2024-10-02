pub mod api;
mod endpoints;
mod errors;
mod macros;
mod models;
mod tests_utils;
mod traits;

#[cfg(test)]
static TEST_AUTH_TOKEN: &'static str = "user";

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
