use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::{self, Utils};

/// Check app request
/// https://api.dropboxapi.com/2/check/app
#[derive(Debug, PartialEq)]
pub struct CheckAppRequest<'a> {
    access_token: &'a str,
    query: &'a str,
}

/// Response struct for check app
#[derive(Deserialize, Debug)]
pub struct CheckAppResponse {
    result: String,
}

/// Implementation of trait for payload
impl utils::Utils for CheckAppRequest<'_> {
    fn payload(&self) -> Option<impl Serialize + Deserialize> {
        Some(HashMap::from([("query", self.query)]))
    }
}

implement_service!(
    CheckAppRequest<'_>,
    CheckAppResponse,
    Endpoint::CheckAppPost,
    vec![Headers::ContentTypeAppJson]
);
