use super::DeleteAllClosedFileRequestsResult;

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
type Request<'a> = DeleteAllClosedFilesRequest<'a>;
type Response = DeleteAllClosedFilesResponse;
type RequestPayload = ();
type ResponsePayload = DeleteAllClosedFileRequestsResult;

/// Delete all closed
/// https://www.dropbox.com/developers/documentation/http/documentation#file_requests-delete_all_closed
#[derive(Debug)]
pub struct DeleteAllClosedFilesRequest<'a> {
    access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DeleteAllClosedFilesResponse {
    payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FileRequestsDeleteAllClosedPost,
    vec![Headers::TestAuthorization]
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
        Endpoint::FileRequestsDeleteAllClosedPost,
        vec![Headers::TestAuthorization],
        Request,
        RequestPayload
    );
}
