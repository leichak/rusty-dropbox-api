use super::{PathWithPropertyGroups, PropertyTemplateWithTaggedType, TemplateId};

use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-templates-get_for_user
#[derive(Debug)]
pub struct TemplatesGetForUserRequest<'a> {
    access_token: &'a str,
    payload: Option<TemplateId>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct TemplatesGetForUserResponse {
    payload: PropertyTemplateWithTaggedType,
}

// Impl utils trait
implement_utils!(TemplatesGetForUserRequest<'_>, TemplateId);

// Impl service trait
implement_service!(
    TemplatesGetForUserRequest<'_>,
    TemplatesGetForUserResponse,
    Endpoint::FilePropertiesTemplatesGetForUserPost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{TemplateId, TemplatesGetForUserRequest};

    use anyhow::Result;
    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests, mockito, Endpoint, Headers, Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesTemplatesGetForUserPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        TemplatesGetForUserRequest,
        TemplateId
    );
}
