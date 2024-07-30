use super::{Cursor, MatchesWithPropertyGroups};

use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, implement_utils, ApiError, AsyncClient, BoxFuture,
    Endpoint, Headers, Service, SyncClient, Utils,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-search-continue
#[derive(Debug)]
pub struct PropertiesSearchContinueRequest<'a> {
    access_token: &'a str,
    payload: Option<Cursor>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
pub struct PropertiesSearchContinueResponse {
    payload: MatchesWithPropertyGroups,
}

// Impl utils trait
implement_utils!(PropertiesSearchContinueRequest<'_>, Cursor);

// Impl service trait
implement_service!(
    PropertiesSearchContinueRequest<'_>,
    PropertiesSearchContinueResponse,
    MatchesWithPropertyGroups,
    Endpoint::FilePropertiesPropertiesSearchContinuePost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::{Cursor, PropertiesSearchContinueRequest};

    use anyhow::Result;
    use api::{
        get_endpoint_test_body_response, get_endpoint_url, get_mut_or_init, get_mut_or_init_async,
        implement_tests, mockito, Endpoint, Headers, Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesPropertiesSearchContinuePost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        PropertiesSearchContinueRequest,
        Cursor
    );
}
