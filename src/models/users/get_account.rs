use super::{BasicAccount, GetAccountArg};

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

type Request<'a> = GetAccountRequest<'a>;
type Response = GetAccountResponse;
type RequestPayload = GetAccountArg;
type ResponsePayload = BasicAccount;

/// Get account
/// <https://www.dropbox.com/developers/documentation/http/documentation#users-get_account>
#[derive(Debug)]
pub struct GetAccountRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetAccountResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::UsersGetAccountPost,
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
        Endpoint::UsersGetAccountPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
