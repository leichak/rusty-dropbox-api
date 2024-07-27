use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::{self, Utils};

/// Delete manual contacts batch request
/// https://api.dropboxapi.com/2/contacts/delete_manual_contacts_batch
#[derive(Debug, PartialEq)]
pub struct DeleteManualContactsBatchRequest<'a> {
    access_token: &'a str,
    email_addresses: Vec<&'a str>,
}

/// Response struct for deleting manual contacts batch
#[derive(Deserialize, Debug)]
pub struct DeleteManualContactsBatchResponse();

/// Implementation of trait for payload
impl utils::Utils for DeleteManualContactsBatchRequest<'_> {
    fn payload(&self) -> Option<impl Serialize + Deserialize> {
        let payload = HashMap::from([("email_addresses", self.email_addresses.clone())]);
        Some(payload)
    }
}

implement_service!(
    DeleteManualContactsBatchRequest<'_>,
    DeleteManualContactsBatchResponse,
    Endpoint::ContactsDeleteManualContactsBatchPost,
    vec![Headers::ContentTypeAppJson]
);
