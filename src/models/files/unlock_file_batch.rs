use super::{LockFileBatchResult as RequestResult, UnlockFileBatchArg as Args};

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

/// Type aliases for readabilitydTag
type Request<'a> = UnlockFileBatchRequest<'a>;
type Response = UnlockFileBatchResponse;
type RequestPayload = Args;
type ResponsePayload = RequestResult;

/// Unlock file batch
/// <https://www.dropbox.com/developers/documentation/http/documentation#files-unlock_file_batch>
#[derive(Debug)]
pub struct UnlockFileBatchRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UnlockFileBatchResponse {
    pub payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilesUnlockFileBatchPost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use crate::TEST_AUTH_TOKEN;

    use super::{Request, RequestPayload};

    use tokio;

    use crate::{
        endpoints::{get_endpoint_url, headers::Headers, Endpoint},
        implement_tests,
        tests_utils::get_endpoint_test_body_response,
        traits::Service,
    };

    implement_tests!(
        Endpoint::FilesUnlockFileBatchPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
