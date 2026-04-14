use super::{GetSharedLinkMetadataArg, SharedLinkMetadata};

use crate::{
    anyhow::Result,
    endpoints::headers::Headers,
    endpoints::{get_endpoint_url, Endpoint},
    errors::ApiError,
    implement_content_upload_utils, implement_download_service,
    traits::{Service, Utils},
    AsyncClient, SyncClient,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

type Request<'a> = GetSharedLinkFileRequest<'a>;
type Response = GetSharedLinkFileResponse;
type RequestPayload = GetSharedLinkMetadataArg;
type ResponsePayload = SharedLinkMetadata;

/// `get_shared_link_file` — downloads the file behind a shared link. Metadata
/// arrives in the `Dropbox-API-Result` response header; bytes in the body.
/// <https://www.dropbox.com/developers/documentation/http/documentation#sharing-get_shared_link_file>
#[derive(Debug)]
pub struct GetSharedLinkFileRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
    /// Unused for download requests but required by the Utils trait shared
    /// with upload endpoints.
    pub data: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetSharedLinkFileResponse {
    pub payload: ResponsePayload,
    pub data: Vec<u8>,
}

implement_content_upload_utils!(Request<'_>, RequestPayload);

implement_download_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::SharingGetSharedLinkFilePost,
    vec![
        Headers::DropboxApiArg("".to_string()),
        Headers::DropboxApiResult,
    ]
);


#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use crate::TEST_AUTH_TOKEN;

    use super::{Request, RequestPayload};

    use tokio;

    use crate::{
        endpoints::{get_endpoint_url, headers::Headers, Endpoint},
        implement_tests,
        tests_utils::get_endpoint_test_body_response,
        traits::Service,
    };

    implement_tests!(
        Endpoint::SharingGetSharedLinkFilePost,
        vec![
            Headers::TestAuthorization,
            Headers::DropboxApiArg("".to_string()),
            Headers::DropboxApiResult,
        ],
        Request,
        RequestPayload
    );
}
