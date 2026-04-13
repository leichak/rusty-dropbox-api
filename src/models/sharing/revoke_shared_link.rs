use super::RevokeSharedLinkArg;

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

type Request<'a> = RevokeSharedLinkRequest<'a>;
type Response = RevokeSharedLinkResponse;
type RequestPayload = RevokeSharedLinkArg;
/// `revoke_shared_link` returns `null` on success. Modelled as
/// `serde_json::Value` to accept null without gymnastics.
type ResponsePayload = serde_json::Value;

/// Revoke shared link
/// <https://www.dropbox.com/developers/documentation/http/documentation#sharing-revoke_shared_link>
#[derive(Debug)]
pub struct RevokeSharedLinkRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RevokeSharedLinkResponse {
    pub payload: ResponsePayload,
}

implement_utils!(Request<'_>, RequestPayload);

implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SharingRevokeSharedLinkPost,
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
        tests_utils::{get_endpoint_test_body_response},
        traits::Service,
    };

    implement_tests!(
        Endpoint::SharingRevokeSharedLinkPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
