use super::TemplateIds;

use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-templates-list_for_user
#[derive(Debug)]
pub struct TemplatesListForUserRequest<'a> {
    access_token: &'a str,
    payload: Option<()>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct TemplatesListForUserResponse {
    payload: TemplateIds,
}

// Impl utils trait
implement_utils!(TemplatesListForUserRequest<'_>, ());

// Impl service trait
implement_service!(
    TemplatesListForUserRequest<'_>,
    TemplatesListForUserResponse,
    Endpoint::FilePropertiesTemplatesListForUserPost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use anyhow::Result;
    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests, mockito, Endpoint, Headers, Service,
    };
    use tokio;

    use super::TemplatesListForUserRequest;

    implement_tests!(
        Endpoint::FilePropertiesTemplatesListForUserPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        TemplatesListForUserRequest,
        ()
    );
}
