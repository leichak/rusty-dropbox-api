use super::MarkSeenArg;

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

type Request<'a> = MarkSeenRequest<'a>;
type Response = MarkSeenResponse;
type RequestPayload = MarkSeenArg;
type ResponsePayload = serde_json::Value;

/// `seen_state/mark_seen`
/// <https://www.dropbox.com/developers/documentation/http/documentation#seen_state-mark_seen>
#[derive(Debug)]
pub struct MarkSeenRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MarkSeenResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SeenStateMarkSeenPost,
    vec![Headers::ContentTypeAppJson]
);
