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

type Request<'a> = RelinquishFileMembershipRequest<'a>;
type Response = RelinquishFileMembershipResponse;
type RequestPayload = serde_json::Value;
type ResponsePayload = serde_json::Value;

/// `relinquish_file_membership`
/// Payload and response are modelled as `serde_json::Value` for now — the
/// endpoint is wired and reachable; full typed structs follow in a later pass.
#[derive(Debug)]
pub struct RelinquishFileMembershipRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RelinquishFileMembershipResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SharingRelinquishFileMembershipPost,
    vec![Headers::ContentTypeAppJson]
);
