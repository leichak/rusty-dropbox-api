use super::{ResendVerificationEmailArg, ResendVerificationEmailResult};

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

type Request<'a> = ResendVerificationEmailsRequest<'a>;
type Response = ResendVerificationEmailsResponse;
type RequestPayload = ResendVerificationEmailArg;
type ResponsePayload = ResendVerificationEmailResult;

/// `secondary_emails/resend_verification_emails`
/// <https://www.dropbox.com/developers/documentation/http/documentation#secondary_emails-resend_verification_emails>
#[derive(Debug)]
pub struct ResendVerificationEmailsRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ResendVerificationEmailsResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SecondaryEmailsResendVerificationEmailsPost,
    vec![Headers::ContentTypeAppJson]
);
