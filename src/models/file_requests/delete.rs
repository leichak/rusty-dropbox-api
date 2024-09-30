use super::{DeleteFileRequestArgs, DeleteFileRequestResult};

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
type Request<'a> = DeleteFilesRequest<'a>;
type Response = DeleteFilesResponse;
type RequestPayload = DeleteFileRequestArgs;
type ResponsePayload = DeleteFileRequestResult;

/// Delete file
/// https://www.dropbox.com/developers/documentation/http/documentation#file_requests-delete
#[derive(Debug)]
pub struct DeleteFilesRequest<'a> {
    access_token: &'a str,
    payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DeleteFilesResponse {
    payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FileRequestsDeletePost,
    vec![Headers::TestAuthorization]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{Request, RequestPayload};

    use tokio;

    use crate::{
        endpoints::{get_endpoint_url, headers::Headers, Endpoint},
        implement_tests,
        tests_utils::{get_endpoint_test_body_response, get_mut_or_init, get_mut_or_init_async},
        traits::Service,
    };

    implement_tests!(
        Endpoint::FileRequestsDeletePost,
        vec![Headers::TestAuthorization],
        Request,
        RequestPayload
    );
}
