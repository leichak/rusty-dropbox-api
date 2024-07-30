use super::PathWithTemplateIds;

use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-remove
#[derive(Debug)]
pub struct PropertiesRemoveRequest<'a> {
    access_token: &'a str,
    payload: Option<PathWithTemplateIds>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct PropertiesRemoveResponse {
    payload: (),
}

// Impl utils trait
implement_utils!(PropertiesRemoveRequest<'_>, PathWithTemplateIds);

// Impl service trait
implement_service!(
    PropertiesRemoveRequest<'_>,
    PropertiesRemoveResponse,
    PathWithTemplateIds,
    Endpoint::FilePropertiesPropertiesRemovePost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{PathWithTemplateIds, PropertiesRemoveRequest};

    use anyhow::Result;
    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests, mockito, Endpoint, Headers, Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesPropertiesRemovePost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        PropertiesRemoveRequest,
        PathWithTemplateIds
    );
}
