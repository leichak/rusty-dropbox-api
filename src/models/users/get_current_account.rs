use super::FullAccount;

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

/// `/users/get_current_account` takes no body. The payload type is `()` to
/// match the Request-struct shape used across the crate.
type Request<'a> = GetCurrentAccountRequest<'a>;
type Response = GetCurrentAccountResponse;
type RequestPayload = ();
type ResponsePayload = FullAccount;

/// Get current account
/// <https://www.dropbox.com/developers/documentation/http/documentation#users-get_current_account>
#[derive(Debug)]
pub struct GetCurrentAccountRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetCurrentAccountResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::UsersGetCurrentAccountPost,
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
        tests_utils::{get_endpoint_test_body_response, get_mut_or_init, get_mut_or_init_async},
        traits::Service,
    };

    implement_tests!(
        Endpoint::UsersGetCurrentAccountPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
