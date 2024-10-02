use super::{MatchesWithPropertyGroups, QueriesWithTemplateFilter};

use crate::{
    anyhow::Result,
    endpoints::headers::Headers,
    endpoints::{get_endpoint_url, Endpoint},
    errors::ApiError,
    implement_service, implement_utils,
    traits::{Service, Utils},
    AsyncClient, BoxFuture, SyncClient,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Type aliases for readability
type Request<'a> = PropertiesSearchRequest<'a>;
type Response = PropertiesSearchResponse;
type RequestPayload = QueriesWithTemplateFilter;
type ResponsePayload = MatchesWithPropertyGroups;

/// Add properties struct for file request
/// https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-search
#[derive(Debug)]
pub struct PropertiesSearchRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PropertiesSearchResponse {
    pub payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilePropertiesPropertiesSearchPost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_AUTH_TOKEN;

    use super::{Request, RequestPayload};

    use crate::{
        endpoints::{get_endpoint_url, headers::Headers, Endpoint},
        implement_tests,
        tests_utils::{get_endpoint_test_body_response, get_mut_or_init, get_mut_or_init_async},
        traits::Service,
    };
    use tokio;

    implement_tests!(
        Endpoint::FilePropertiesPropertiesSearchPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
