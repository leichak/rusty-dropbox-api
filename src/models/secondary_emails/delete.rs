use super::{DeleteSecondaryEmailsArg, DeleteSecondaryEmailsResult};

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

type Request<'a> = DeleteSecondaryEmailsRequest<'a>;
type Response = DeleteSecondaryEmailsResponse;
type RequestPayload = DeleteSecondaryEmailsArg;
type ResponsePayload = DeleteSecondaryEmailsResult;

/// `secondary_emails/delete`
/// <https://www.dropbox.com/developers/documentation/http/documentation#secondary_emails-delete>
#[derive(Debug)]
pub struct DeleteSecondaryEmailsRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DeleteSecondaryEmailsResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SecondaryEmailsDeletePost,
    vec![Headers::ContentTypeAppJson]
);
