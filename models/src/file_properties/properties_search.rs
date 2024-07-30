use super::{MatchesWithPropertyGroups, QueriesWithTemplateFilter};

use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-search
#[derive(Debug)]
pub struct PropertiesSearchRequest<'a> {
    access_token: &'a str,
    payload: Option<QueriesWithTemplateFilter>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct PropertiesSearchResponse {
    payload: MatchesWithPropertyGroups,
}

// Impl utils trait
implement_utils!(PropertiesSearchRequest<'_>, QueriesWithTemplateFilter);

// Impl service trait
implement_service!(
    PropertiesSearchRequest<'_>,
    PropertiesSearchResponse,
    MatchesWithPropertyGroups,
    Endpoint::FilePropertiesPropertiesSearchPost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{PropertiesSearchRequest, QueriesWithTemplateFilter};

    use anyhow::Result;
    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests, mockito, Endpoint, Headers, Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesPropertiesSearchPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        PropertiesSearchRequest,
        QueriesWithTemplateFilter
    );
}
