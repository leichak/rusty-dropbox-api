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

type Request<'a> = ListFolderMembersContinueRequest<'a>;
type Response = ListFolderMembersContinueResponse;
type RequestPayload = super::ListFolderMembersContinueArg;
type ResponsePayload = serde_json::Value;

/// `list_folder_members_continue`
/// Payload and response are modelled as `serde_json::Value` for now — the
/// endpoint is wired and reachable; full typed structs follow in a later pass.
#[derive(Debug)]
pub struct ListFolderMembersContinueRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListFolderMembersContinueResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SharingListFolderMembersContinuePost,
    vec![Headers::ContentTypeAppJson]
);
