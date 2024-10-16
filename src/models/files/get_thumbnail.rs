use super::{GetThumbnailResult as RequestResult, ThumbnailV2Arg as Args};

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
type Request<'a> = GetThumbnailRequest<'a>;
type Response = GetThumbnailResponse;
type RequestPayload = Args;
type ResponsePayload = RequestResult;

/// Get thumbnail v2
/// https://content.dropboxapi.com/2/files/get_thumbnail_v2
#[derive(Debug)]
pub struct GetThumbnailRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetThumbnailResponse {
    pub payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilesGetThumbnailPost,
    vec![Headers::DropboxApiArg("".to_string())]
);

#[cfg(test)]
mod tests {
    use crate::TEST_AUTH_TOKEN;

    use super::{Request, RequestPayload};

    use tokio;

    use crate::{
        endpoints::{get_endpoint_url, headers::Headers, Endpoint},
        implement_tests,
        tests_utils::{get_endpoint_test_body_response, get_mut_or_init, get_mut_or_init_async},
        traits::Service,
    };

    implement_tests!(
        Endpoint::FilesGetThumbnailPost,
        vec![
            Headers::TestAuthorization,
            Headers::DropboxApiArg("".to_string())
        ],
        Request,
        RequestPayload
    );
}
