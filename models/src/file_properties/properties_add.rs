use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use super::PathWithPropertyGroups;

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::{self, Utils};

/// Add properties for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-add
#[derive(Debug)]
pub struct PropertiesAddRequest<'a> {
    access_token: &'a str,
    path_with_property_groups: PathWithPropertyGroups,
}

/// Response struct for adding properties of file
#[derive(Deserialize, Debug)]
pub struct PropertiesAddResponse;

/// Implementation of trait for payload
impl utils::Utils for PropertiesAddRequest<'_> {
    fn payload(&self) -> Option<impl Serialize + Deserialize> {
        Some(self.payload)
    }
}

implement_service!(
    PropertiesAddRequest<'_>,
    PropertiesAddResponse,
    Endpoint::FilePropertiesPropertiesAddPost,
    vec![Headers::ContentTypeAppJson]
);
