use super::PathWithUpdatePropertyGroups;

use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-update
#[derive(Debug)]
pub struct PropertiesUpdateRequest<'a> {
    access_token: &'a str,
    payload: Option<PathWithUpdatePropertyGroups>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct PropertiesUpdateResponse {
    payload: (),
}

type Request<'a> = PropertiesUpdateRequest<'a>;
type Response = PropertiesUpdateResponse;
type RequestPayload = PathWithUpdatePropertyGroups;
type ResponsePayload = ();

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilePropertiesPropertiesUpdatePost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{Request, RequestPayload};

    use anyhow::Result;
    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests,
        mockito::{self},
        Endpoint, Headers, Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesPropertiesUpdatePost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
