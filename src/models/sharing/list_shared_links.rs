use super::{ListSharedLinksArg, ListSharedLinksResult};

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

type Request<'a> = ListSharedLinksRequest<'a>;
type Response = ListSharedLinksResponse;
type RequestPayload = ListSharedLinksArg;
type ResponsePayload = ListSharedLinksResult;

/// List shared links
/// <https://www.dropbox.com/developers/documentation/http/documentation#sharing-list_shared_links>
#[derive(Debug)]
pub struct ListSharedLinksRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListSharedLinksResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SharingListSharedLinksPost,
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
        Endpoint::SharingListSharedLinksPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
