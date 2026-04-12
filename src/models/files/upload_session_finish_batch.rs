
use super::{UploadSessionFinishBatchArg, UploadSessionFinishBatchLaunch};

use crate::{
    anyhow::Result,
    endpoints::headers::Headers,
    endpoints::{get_endpoint_url, Endpoint},
    errors::ApiError,
    implement_service, implement_utils,
    traits::{Service, Utils},
    AsyncClient, BoxFuture, SyncClient,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Type aliases for readability
type Request<'a> = UploadSessionFinishBatchRequest<'a>;
type Response = UploadSessionFinishBatchResponse;
type RequestPayload = UploadSessionFinishBatchArg;
type ResponsePayload = UploadSessionFinishBatchLaunch;

/// UploadSessionFinishBatch
/// https://www.dropbox.com/developers/documentation/http/documentation#files-UploadSessionFinishBatch
#[derive(Debug)]
pub struct UploadSessionFinishBatchRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UploadSessionFinishBatchResponse {
    pub payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilesUploadSessionFinishBatchPost,
    vec![
        Headers::ContentTypeAppOctetStream("".to_string()),
        Headers::DropboxApiArg("".to_string())
    ]
);

#[cfg(test)]
mod tests {
    use crate::TEST_AUTH_TOKEN;

    use super::{Request, RequestPayload};

    use tokio;

    use crate::{
        endpoints::{get_endpoint_url, headers::Headers, Endpoint},
        implement_tests,
        tests_utils::{get_endpoint_test_body_response, get_mut_or_init, get_mut_or_init_async},
        traits::Service,
    };

    implement_tests!(
        Endpoint::FilesUploadSessionFinishBatchPost,
        vec![
            Headers::TestAuthorization,
            Headers::ContentTypeAppOctetStream("".to_string()),
            Headers::DropboxApiArg("".to_string()),
        ],
        Request,
        RequestPayload
    );
}
