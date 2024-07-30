use super::PathWithPropertyGroups;

use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

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

implement_utils!(PropertiesAddRequest<'_>, PathWithPropertyGroups);

implement_service!(
    PropertiesAddRequest<'_>,
    PropertiesAddResponse,
    Endpoint::FilePropertiesPropertiesAddPost,
    vec![Headers::ContentTypeAppJson]
);
