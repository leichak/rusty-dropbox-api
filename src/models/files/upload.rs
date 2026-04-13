use super::{FileMetadata, UploadArg};

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
type Request<'a> = UploadRequest<'a>;
type Response = UploadResponse;
type RequestPayload = UploadArg;
type ResponsePayload = FileMetadata;

/// Upload
/// https://www.dropbox.com/developers/documentation/http/documentation#files-upload
#[derive(Debug)]
pub struct UploadRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
    /// Binary file contents — travels as the raw HTTP body.
    pub data: Option<Vec<u8>>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UploadResponse {
    pub payload: ResponsePayload,
}

// Impl utils trait (with content_body forwarding the data field)
implement_content_upload_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilesUploadPost,
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
        Endpoint::FilesUploadPost,
        vec![
            Headers::TestAuthorization,
            Headers::ContentTypeAppOctetStream,
            Headers::DropboxApiArg("".to_string()),
        ],
        Request,
        RequestPayload
    );
}
