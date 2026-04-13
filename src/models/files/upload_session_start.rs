
use super::{UploadSessionStartArg, UploadSessionStartResult};

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

/// Type aliases for readability
type Request<'a> = UploadSessionStartRequest<'a>;
type Response = UploadSessionStartResponse;
type RequestPayload = UploadSessionStartArg;
type ResponsePayload = UploadSessionStartResult;

/// UploadSessionStart
/// https://www.dropbox.com/developers/documentation/http/documentation#files-UploadSessionStart
#[derive(Debug)]
pub struct UploadSessionStartRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
    /// Binary body bytes.
    pub data: Option<Vec<u8>>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UploadSessionStartResponse {
    pub payload: ResponsePayload,
}

// Impl utils trait
implement_content_upload_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilesUploadSessionStartPost,
    vec![
        Headers::ContentTypeAppOctetStream,
        Headers::DropboxApiArg("".to_string())
    ]
);

#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use crate::TEST_AUTH_TOKEN;

    use super::{Request, RequestPayload};

    use tokio;

    use crate::{
        endpoints::{get_endpoint_url, headers::Headers, Endpoint},
        implement_tests,
        tests_utils::{get_endpoint_test_body_response},
        traits::Service,
    };

    implement_tests!(
        Endpoint::FilesUploadSessionStartPost,
        vec![
            Headers::TestAuthorization,
            Headers::ContentTypeAppOctetStream,
            Headers::DropboxApiArg("".to_string()),
        ],
        Request,
        RequestPayload
    );
}
