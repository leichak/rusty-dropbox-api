use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use super::PathWithPropertyGroups;

use serde::Deserialize;

use std::{future::Future, pin::Pin};

use crate::utils::{self, Utils};

/// Add properties for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-add
#[derive(Debug)]
pub struct PropertiesAddRequest<'a> {
    access_token: &'a str,
    payload: Option<PathWithPropertyGroups>,
}

/// Response struct for adding properties of file
#[derive(Deserialize, Debug)]
pub struct PropertiesAddResponse;

/// Implementation of trait for payload
impl utils::Utils<'_> for PropertiesAddRequest<'_> {
    type A = PathWithPropertyGroups;
    fn payload(&self) -> Option<&Self::A> {
        if self.payload.is_some() {
            return Some(self.payload.as_ref().unwrap());
        }
        None
    }
}

implement_service!(
    PropertiesAddRequest<'_>,
    PropertiesAddResponse,
    Endpoint::FilePropertiesPropertiesAddPost,
    vec![Headers::ContentTypeAppJson]
);
