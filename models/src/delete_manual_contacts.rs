use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::{
    strum_macros::AsRefStr,
    utils::{self, Utils},
};

/// Delete manual contacts request
/// https://www.dropbox.com/developers/documentation/http/documentation#contacts-delete_manual_contacts
#[derive(Debug, PartialEq)]
pub struct DeleteManualContactsRequest<'a> {
    access_token: &'a str,
}

/// Response struct for deleting manual contacts
#[derive(Deserialize, Debug)]
pub struct DeleteManualContactsResponse();

/// Implementation of trait for payload
impl utils::Utils for DeleteManualContactsRequest<'_> {
    fn payload(&self) -> Option<impl Serialize + Deserialize> {
        None::<()>
    }
}

implement_service!(
    DeleteManualContactsRequest<'_>,
    DeleteManualContactsResponse,
    Endpoint::ContactsDeleteManualContactsPost,
    vec![Headers::ContentTypeAppJson]
);
