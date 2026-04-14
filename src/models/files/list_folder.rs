use super::{ListFolderArgs as Args, ListFolderResult as RequestResult};

use crate::{
    anyhow::Result,
    endpoints::headers::Headers,
    endpoints::{get_endpoint_url, Endpoint},
    errors::ApiError,
    implement_service, implement_utils,
    traits::{Service, Utils},
    AsyncClient, SyncClient,
};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// Type aliases for readability
type Request<'a> = ListFolderRequest<'a>;
type Response = ListFolderResponse;
type RequestPayload = Args;
type ResponsePayload = RequestResult;

/// List files
/// <https://www.dropbox.com/developers/documentation/http/documentation#files-list_folder>
#[derive(Debug)]
pub struct ListFolderRequest<'a> {
    pub access_token: &'a str,
    pub payload: Option<RequestPayload>,
}

/// Response struct for adding properties
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListFolderResponse {
    pub payload: ResponsePayload,
}

// Impl utils trait
implement_utils!(Request<'_>, RequestPayload);

// Impl service trait
implement_service!(
    Request<'_>,
    Response,
    ResponsePayload,
    Endpoint::FilesListFolderPost,
    vec![Headers::ContentTypeAppJson]
);

impl<'a> ListFolderRequest<'a> {
    /// Walk every page of list_folder + list_folder/continue and return the
    /// flat list of entries. Avoids callers having to implement the
    /// cursor-follow loop themselves.
    pub async fn collect_all(&self) -> Result<Vec<super::Metadata>> {
        use super::list_folders_continue::ListFolderContinueRequest;
        use super::ListFolderContinueArgs;
        use anyhow::Context;

        let token = self.access_token.to_string();

        let first = self
            .call()
            .await?
            .context("list_folder returned empty response")?;
        let mut all = first.payload.entries;
        let mut cursor = first.payload.cursor;
        let mut has_more = first.payload.has_more;

        while has_more {
            let next_req = ListFolderContinueRequest {
                access_token: &token,
                payload: Some(ListFolderContinueArgs { cursor }),
            };
            let next = next_req
                .call()
                .await?
                .context("list_folder/continue returned empty response")?;
            all.extend(next.payload.entries);
            cursor = next.payload.cursor;
            has_more = next.payload.has_more;
        }

        Ok(all)
    }
}

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
        Endpoint::FilesListFolderPost,
        vec![Headers::TestAuthorization, Headers::ContentTypeAppJson],
        Request,
        RequestPayload
    );
}
