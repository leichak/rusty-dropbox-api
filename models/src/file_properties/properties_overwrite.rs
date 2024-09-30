use super::PathWithPropertyGroups;

use anyhow::Result;
use api::{
    anyhow, consts::get_endpoint_url, consts::Endpoint, implement_service, implement_utils,
    ApiError, AsyncClient, BoxFuture, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Type aliases for readability
type Request<'a> = PropertiesOverwriteRequest<'a>;
type Response = PropertiesOverwriteResponse;
type RequestPayload = PathWithPropertyGroups;
type ResponsePayload = ();

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-overwrite
#[derive(Debug)]
pub struct PropertiesOverwriteRequest<'a> {
    access_token: &'a str,
    payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PropertiesOverwriteResponse {
    payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilePropertiesPropertiesOverwritePost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{Request, RequestPayload};

    use anyhow::Result;
    use api::{
        consts::get_endpoint_test_body_response,
        consts::get_endpoint_url,
        consts::Endpoint,
        get_mut_or_init, get_mut_or_init_async, implement_tests,
        mockito::{self},
        Headers, Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesPropertiesOverwritePost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
