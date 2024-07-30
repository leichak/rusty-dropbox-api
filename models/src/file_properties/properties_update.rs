use super::PathWithPropertyGroups;

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
    payload: Option<PathWithPropertyGroups>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct PropertiesUpdateResponse {
    payload: (),
}

// Impl utils trait
implement_utils!(PropertiesUpdateRequest<'_>, PathWithPropertyGroups);

// Impl service trait
implement_service!(
    PropertiesUpdateRequest<'_>,
    PropertiesUpdateResponse,
    Endpoint::FilePropertiesPropertiesUpdatePost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{PathWithPropertyGroups, PropertiesUpdateRequest};

    use anyhow::Result;
    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests, mockito, Endpoint, Headers, Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesPropertiesUpdatePost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        PropertiesUpdateRequest,
        PathWithPropertyGroups
    );
}
