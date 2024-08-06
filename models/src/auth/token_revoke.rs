use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Type aliases for readability
type Request<'a> = TokenRevokeRequest<'a>;
type Response = TokenRevokeResponse;
type RequestPayload = ();
type ResponsePayload = ();

/// Add properties struct for token revoke
/// https://api.dropboxapi.com/2/auth/token/revoke
#[derive(Debug)]
pub struct TokenRevokeRequest<'a> {
    access_token: &'a str,
    payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct TokenRevokeResponse {
    payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilePropertiesPropertiesAddPost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{Request, RequestPayload};
    use anyhow::Result;
    use tokio;

    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests,
        mockito::{self},
        Endpoint, Headers, Service,
    };

    implement_tests!(
        Endpoint::FilePropertiesPropertiesAddPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
