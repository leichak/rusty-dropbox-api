use super::{AddSecondaryEmailsArg, AddSecondaryEmailsResult};

use crate::{
    anyhow::Result,
    endpoints::headers::Headers,
    endpoints::{get_endpoint_url, Endpoint},
    errors::ApiError,
    implement_service, implement_utils,
    traits::{Service, Utils},
    AsyncClient, SyncClient,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

type Request<'a> = AddSecondaryEmailsRequest<'a>;
type Response = AddSecondaryEmailsResponse;
type RequestPayload = AddSecondaryEmailsArg;
type ResponsePayload = AddSecondaryEmailsResult;

/// `secondary_emails/add`
/// <https://www.dropbox.com/developers/documentation/http/documentation#secondary_emails-add>
#[derive(Debug)]
pub struct AddSecondaryEmailsRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AddSecondaryEmailsResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SecondaryEmailsAddPost,
    vec![Headers::ContentTypeAppJson]
);
