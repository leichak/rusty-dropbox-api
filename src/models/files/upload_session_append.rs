use super::UploadSessionAppendArg;

use crate::{
    anyhow::Result,
    endpoints::headers::Headers,
    endpoints::{get_endpoint_url, Endpoint},
    errors::ApiError,
    implement_content_upload_utils, implement_service,
    traits::{Service, Utils},
    AsyncClient, SyncClient,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

type Request<'a> = UploadSessionAppendRequest<'a>;
type Response = UploadSessionAppendResponse;
type RequestPayload = UploadSessionAppendArg;
/// `upload_session/append_v2` returns null on success.
type ResponsePayload = serde_json::Value;

/// Upload session append v2
/// <https://www.dropbox.com/developers/documentation/http/documentation#files-upload_session-append_v2>
#[derive(Debug)]
pub struct UploadSessionAppendRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
    /// Binary chunk appended to the upload session.
    pub data: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UploadSessionAppendResponse {
    pub payload: ResponsePayload,
}

implement_content_upload_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilesUploadSessionAppendPost,
    vec![
        Headers::ContentTypeAppOctetStream,
        Headers::DropboxApiArg("".to_string()),
    ]
);
