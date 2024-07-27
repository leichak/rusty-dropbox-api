use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::{self, Utils};

/// Check user request
/// https://api.dropboxapi.com/2/check/user
#[derive(Debug, PartialEq)]
pub struct CheckUserRequest<'a> {
    access_token: &'a str,
    query: &'a str,
}

/// Response struct for check user
#[derive(Deserialize, Debug)]
pub struct CheckUserResponse {
    result: String,
}

/// Implementation of trait for payload
impl utils::Utils for CheckUserRequest<'_> {
    fn payload(&self) -> Option<impl Serialize + Deserialize> {
        Some(HashMap::from([("query", self.query)]))
    }
}

implement_service!(
    CheckUserRequest<'_>,
    CheckUserResponse,
    Endpoint::CheckAppPost,
    vec![Headers::ContentTypeAppJson]
);
