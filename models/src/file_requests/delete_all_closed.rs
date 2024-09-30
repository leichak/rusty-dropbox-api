use super::DeleteAllClosedFileRequestsResult;

use anyhow::Result;
use api::{
    anyhow, consts::get_endpoint_url, consts::Endpoint, implement_service, implement_utils,
    ApiError, AsyncClient, BoxFuture, Headers, Service, SyncClient, Utils,
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
    payload: Option<RequestPayload>,
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
    use crate::TEST_TOKEN;

    use super::{Request, RequestPayload};
    use anyhow::Result;
    use tokio;

    use api::{
        consts::get_endpoint_test_body_response,
        consts::get_endpoint_url,
        consts::Endpoint,
        get_mut_or_init, get_mut_or_init_async, implement_tests,
        mockito::{self},
        Headers, Service,
    };

    implement_tests!(
        Endpoint::FileRequestsDeleteAllClosedPost,
        vec![Headers::TestAuthorization],
        Request,
        RequestPayload
    );
}
