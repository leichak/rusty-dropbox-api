pub use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard, OnceLock};

pub use {
    anyhow, anyhow::Result, async_trait, futures::future::BoxFuture, mockito, mockito::Server,
    reqwest, serde_json, tokio,
};

// Clients
lazy_static! {
    pub static ref SyncClient: reqwest::blocking::Client = reqwest::blocking::Client::new();
    pub static ref AsyncClient: reqwest::Client = reqwest::Client::new();
}

/// Auth test token
#[cfg(feature = "test-utils")]
pub static TEST_TOKEN: &'static str = "123456";

/// Test servers urls and ports
const MOCK_SERVER_SYNC_URL: &str = "0.0.0.0";
const MOCK_SERVER_SYNC_PORT: u16 = 1221;
const MOCK_SERVER_ASYNC_URL: &str = "0.0.0.0";
const MOCK_SERVER_ASYNC_PORT: u16 = 1220;

/// Test servers
#[cfg(feature = "test-utils")]
pub static MOCK_SERVER_SYNC: OnceLock<Mutex<Server>> = OnceLock::new();
#[cfg(feature = "test-utils")]
pub static MOCK_SERVER_ASYNC: OnceLock<Mutex<Server>> = OnceLock::new();

/// Sync function that inits default or get mutex to test server
#[cfg(feature = "test-utils")]
pub fn get_mut_or_init() -> MutexGuard<'static, Server> {
    MOCK_SERVER_SYNC
        .get_or_init(|| {
            Mutex::new(mockito::Server::new_with_opts(mockito::ServerOpts {
                host: MOCK_SERVER_SYNC_URL,
                port: MOCK_SERVER_SYNC_PORT,
                assert_on_drop: false,
            }))
        })
        .lock()
        .expect("Failed to lock")
}

#[cfg(feature = "test-utils")]
pub async fn get_mut_or_init_async() -> MutexGuard<'static, Server> {
    MOCK_SERVER_ASYNC
        .get_or_init(|| {
            let server = futures::executor::block_on(mockito::Server::new_with_opts_async(
                mockito::ServerOpts {
                    host: MOCK_SERVER_ASYNC_URL,
                    port: MOCK_SERVER_ASYNC_PORT,
                    assert_on_drop: false,
                },
            ));

            Mutex::new(server)
        })
        .lock()
        .expect("Failed to lock")
}

/// Enum describing set of errors that can occur
/// Thiserror macro to derive std::error::Error trait
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Unknown")] // display trait
    Unknown,
    #[error("Reqwest error {0}")] // display trait
    RequestError(anyhow::Error),
    #[error("Parsing error {0}")] // display trait
    ParsingError(anyhow::Error),
    #[error("Dropbox error {0}")] // display trait
    DropBoxError(anyhow::Error),
}

/// Trait for both sync and async calls
pub trait Service<O: Sized, F: Sized> {
    fn call_sync(&self) -> Result<Option<O>>;
    fn call(&self) -> Result<F>;
}

/// Macro implementing tests
#[macro_export]
macro_rules! implement_tests {
    ($endpoint:expr, $headers:expr, $req:ident, $payload:ty) => {
        #[tokio::test]
        pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
            let (body, response) = get_endpoint_test_body_response($endpoint);

            let mut mock;
            {
                let mut server = get_mut_or_init_async().await;
                let (_, url) = get_endpoint_url($endpoint);

                mock = server.mock("POST", url.unwrap().as_str()).with_status(200);

                let headers: Vec<Headers> = $headers;

                for h in &headers {
                    mock = mock.with_header(h.get_str().0, h.get_str().1);
                }
                if let Some(body) = &body {
                    mock = mock.match_body(mockito::Matcher::JsonString(body.to_string()));
                }
                if let Some(response) = &response {
                    mock = mock.with_body(response);
                }
                mock = mock.create_async().await;
            }

            let payload: Option<$payload>;
            if let Some(body) = body {
                payload = Some(serde_json::from_str(&body).expect("failed to deserialise"));
            } else {
                payload = None;
            }

            let request = $req {
                access_token: &TEST_TOKEN,
                payload,
            };

            let f = request.call()?;
            let _ = f.await?;

            mock.assert();

            Ok(())
        }

        #[test]
        pub fn test_sync_pass() -> Result<(), Box<dyn std::error::Error>> {
            let (body, response) = get_endpoint_test_body_response($endpoint);

            let mut mock;
            {
                let mut server = get_mut_or_init();
                let (_, url) = get_endpoint_url($endpoint);

                mock = server.mock("POST", url.unwrap().as_str()).with_status(200);

                let headers: Vec<Headers> = $headers;

                for h in &headers {
                    mock = mock.with_header(h.get_str().0, h.get_str().1);
                }
                if let Some(body) = &body {
                    mock = mock.match_body(mockito::Matcher::JsonString(body.to_string()));
                }
                if let Some(response) = &response {
                    mock = mock.with_body(response);
                }
                mock = mock.create();
            }

            let payload: Option<$payload>;
            if let Some(body) = body {
                payload = Some(serde_json::from_str(&body).expect("failed to deserialise"));
            } else {
                payload = None;
            }

            let request = $req {
                access_token: &TEST_TOKEN,
                payload,
            };

            let _ = request.call_sync()?;
            mock.assert();

            Ok(())
        }
    };
}

/// Macro implementing Service trait
#[macro_export]
macro_rules! implement_service {
    ($req:ty, $resp:ty, $endpoints:expr, $headers:expr) => {
        impl Service<$resp, BoxFuture<'_, Result<Option<$resp>>>> for $req {
            fn call_sync(&self) -> Result<Option<$resp>> {
                let endpoint = get_endpoint_url($endpoints).0;

                let headers: Vec<Headers> = $headers;

                let mut response = SyncClient.post(endpoint).bearer_auth(self.access_token);

                for h in &headers {
                    response = response.header(h.get_str().0, h.get_str().1);
                }

                if let Some(payload) = self.payload() {
                    response = response.json(payload);
                }

                let response = response
                    .send()
                    .map_err(|err| ApiError::RequestError(err.into()))?;

                match response.error_for_status() {
                    Ok(response) => {
                        let text = response
                            .text()
                            .map_err(|err| ApiError::ParsingError(err.into()))?;

                        if text.is_empty() {
                            return Ok(None);
                        }

                        let response: $resp = serde_json::from_str(&text)
                            .map_err(|err| ApiError::ParsingError(err.into()))?;
                        Ok(Some(response))
                    }
                    Err(err) => Err(ApiError::DropBoxError(err.into()).into()),
                }
            }

            fn call(&self) -> Result<Pin<Box<dyn Future<Output = Result<Option<$resp>>> + Send>>> {
                let mut endpoint = get_endpoint_url($endpoints).0;
                if let Some(url) = get_endpoint_url($endpoints).1 {
                    endpoint = url;
                }

                let headers: Vec<Headers> = $headers;

                let mut response = AsyncClient.post(endpoint).bearer_auth(self.access_token);

                for h in &headers {
                    response = response.header(h.get_str().0, h.get_str().1);
                }

                if let Some(payload) = &self.payload() {
                    response = response.json(payload);
                }

                let response = response.send();

                let block = async {
                    let response = response
                        .await
                        .map_err(|err| ApiError::RequestError(err.into()))?;

                    let response = response
                        .error_for_status()
                        .map_err(|err| ApiError::DropBoxError(err.into()))?;

                    let text = response
                        .text()
                        .await
                        .map_err(|err| ApiError::ParsingError(err.into()))?;

                    if text.is_empty() {
                        return Ok(None);
                    }

                    let response: $resp = serde_json::from_str(&text)
                        .map_err(|err| ApiError::ParsingError(err.into()))?;

                    Result::<Option<$resp>>::Ok(Some(response))
                };
                Ok(Box::pin(block))
            }
        }
    };
}

/// Enum representing api available endpoints
/// It is passed to fhe function
#[derive(Debug)]
pub enum Endpoint {
    CheckAppPost,
    CountPost,
    CreatePost,
    DeleteAllClosedPost,
    ContactsDeleteManualContactsBatchPost,
    ContactsDeleteManualContactsPost,
    DeletePost,
    AddFolderMemberPost,
    FilesDownloadPost,
    FilesUploadPost,
    FilesUploadSessionAppendV2Post,
    FilesUploadSessionFinishPost,
    FilesUploadSessionStartPost,
    GetPost,
    ListContinuePost,
    ListPost,
    FilePropertiesPropertiesAddPost,
    FilePropertiesPropertiesOverwritePost,
    FilePropertiesPropertiesRemovePost,
    FilePropertiesPropertiesSearchContinuePost,
    FilePropertiesPropertiesSearchPost,
    FilePropertiesPropertiesUpdatePost,
    SetProfilePhotoPost,
    FilePropertiesTemplatesAddForUserPost,
    FilePropertiesTemplatesGetForUserPost,
    FilePropertiesTemplatesListForUserPost,
    FilePropertiesTemplatesRemoveForUserPost,
    FilePropertiesTemplatesUpdateForUserPost,
    TokenRevokePost,
    UpdatePost,
    CheckUserPost,
    UsersFeaturesGetValuesPost,
    UsersGetAccountBatchPost,
    UsersGetAccountPost,
    UsersGetCurrentAccountPost,
    UsersGetSpaceUsagePost,
}

pub fn get_endpoint_url(endpoint: Endpoint) -> (String, Option<String>) {
    let url = match endpoint {
        Endpoint::AddFolderMemberPost => "https://api.dropboxapi.com/2/sharing/add_folder_member",
        Endpoint::CheckAppPost => "https://api.dropboxapi.com/2/check/app",
        Endpoint::CountPost => "https://api.dropboxapi.com/2/file_requests/count",
        Endpoint::CreatePost => "https://api.dropboxapi.com/2/file_requests/create",
        Endpoint::DeleteAllClosedPost => "https://api.dropboxapi.com/2/delete_all_closed",
        Endpoint::ContactsDeleteManualContactsBatchPost => {
            "https://api.dropboxapi.com/2/contacts/delete_manual_contacts_batch"
        }
        Endpoint::ContactsDeleteManualContactsPost => {
            "https://api.dropboxapi.com/2/contacts/delete_manual_contacts"
        }
        Endpoint::DeletePost => "https://api.dropboxapi.com/2/delete",
        Endpoint::FilesDownloadPost => "https://content.dropboxapi.com/2/files/download",
        Endpoint::FilesUploadPost => "https://content.dropboxapi.com/2/files/upload",
        Endpoint::FilesUploadSessionAppendV2Post => {
            "https://content.dropboxapi.com/2/files/upload_session/append_v2"
        }
        Endpoint::FilesUploadSessionFinishPost => {
            "https://content.dropboxapi.com/2/files/upload_session/finish"
        }
        Endpoint::FilesUploadSessionStartPost => {
            "https://content.dropboxapi.com/2/files/upload_session/start"
        }
        Endpoint::GetPost => "https://api.dropboxapi.com/2/get",
        Endpoint::ListContinuePost => "https://api.dropboxapi.com/2/list/continue",
        Endpoint::ListPost => "https://api.dropboxapi.com/2/list",
        Endpoint::FilePropertiesPropertiesAddPost => {
            "https://api.dropboxapi.com/2/file_properties/properties/add"
        }
        Endpoint::FilePropertiesPropertiesOverwritePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/overwrite"
        }
        Endpoint::FilePropertiesPropertiesRemovePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/remove"
        }
        Endpoint::FilePropertiesPropertiesSearchContinuePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/search/continue"
        }
        Endpoint::FilePropertiesPropertiesSearchPost => {
            "https://api.dropboxapi.com/2/file_properties/properties/search"
        }
        Endpoint::FilePropertiesPropertiesUpdatePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/update"
        }
        Endpoint::SetProfilePhotoPost => "https://api.dropboxapi.com/2/account/set_profile_photo",
        Endpoint::FilePropertiesTemplatesAddForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/add_for_user"
        }
        Endpoint::FilePropertiesTemplatesGetForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/get_for_user"
        }
        Endpoint::FilePropertiesTemplatesListForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/list_for_user"
        }
        Endpoint::FilePropertiesTemplatesRemoveForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/remove_for_user"
        }
        Endpoint::FilePropertiesTemplatesUpdateForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/update_for_user"
        }
        Endpoint::TokenRevokePost => "https://api.dropboxapi.com/2/auth/token/revoke",
        Endpoint::UpdatePost => "https://api.dropboxapi.com/2/update",
        Endpoint::CheckUserPost => "https://api.dropboxapi.com/2/check/user",
        Endpoint::UsersFeaturesGetValuesPost => {
            "https://api.dropboxapi.com/2/users/features/get_values"
        }
        Endpoint::UsersGetAccountBatchPost => {
            "https://api.dropboxapi.com/2/users/get_account_batch"
        }
        Endpoint::UsersGetAccountPost => "https://api.dropboxapi.com/2/users/get_account",
        Endpoint::UsersGetCurrentAccountPost => {
            "https://api.dropboxapi.com/2/users/get_current_account"
        }
        Endpoint::UsersGetSpaceUsagePost => "https://api.dropboxapi.com/2/users/get_space_usage",
    };

    let binding: (String, Option<String>) = (url.to_string(), None);
    #[cfg(feature = "test-utils")]
    let binding = test_url(url);

    binding
}

#[cfg(feature = "test-utils")]
pub fn get_endpoint_test_body_response(
    endpoint: Endpoint,
) -> (Option<&'static str>, Option<&'static str>) {
    match endpoint {
        Endpoint::CheckAppPost => todo!(),
        Endpoint::CountPost => todo!(),
        Endpoint::CreatePost => todo!(),
        Endpoint::DeleteAllClosedPost => todo!(),
        Endpoint::ContactsDeleteManualContactsBatchPost => todo!(),
        Endpoint::ContactsDeleteManualContactsPost => todo!(),
        Endpoint::DeletePost => todo!(),
        Endpoint::AddFolderMemberPost => todo!(),
        Endpoint::FilesDownloadPost => todo!(),
        Endpoint::FilesUploadPost => todo!(),
        Endpoint::FilesUploadSessionAppendV2Post => todo!(),
        Endpoint::FilesUploadSessionFinishPost => todo!(),
        Endpoint::FilesUploadSessionStartPost => todo!(),
        Endpoint::GetPost => todo!(),
        Endpoint::ListContinuePost => todo!(),
        Endpoint::ListPost => todo!(),
        Endpoint::FilePropertiesPropertiesAddPost => (
            Some(
                r##"{
            "path": "/my_awesome/word.docx",
            "property_groups": [
                {
                    "fields": [
                        {
                            "name": "Security Policy",
                            "value": "Confidential"
                        }
                    ],
                    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
                }
            ]
        }"##,
            ),
            None,
        ),
        Endpoint::FilePropertiesPropertiesOverwritePost => (
            Some(
                r##"{
    "path": "/my_awesome/word.docx",
    "property_groups": [
        {
            "fields": [
                {
                    "name": "Security Policy",
                    "value": "Confidential"
                }
            ],
            "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
        }
    ]
}"##,
            ),
            None,
        ),
        Endpoint::FilePropertiesPropertiesRemovePost => (
            Some(
                r##"{
    "path": "/my_awesome/word.docx",
    "property_template_ids": [
        "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
    ]
}"##,
            ),
            None,
        ),
        Endpoint::FilePropertiesPropertiesSearchContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}
"##,
            ),
            Some(
                r##"{
    "matches": [
        {
            "id": "id:a4ayc_80_OEAAAAAAAAAXz",
            "is_deleted": false,
            "path": "/my_awesome/word.docx",
            "property_groups": [
                {
                    "fields": [
                        {
                            "name": "Security Policy",
                            "value": "Confidential"
                        }
                    ],
                    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
                }
            ]
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilePropertiesPropertiesSearchPost => (
            Some(
                r##"{
    "queries": [
        {
            "logical_operator": "or_operator",
            "mode": {
                ".tag": "field_name",
                "field_name": "Security"
            },
            "query": "Confidential"
        }
    ],
    "template_filter": "filter_none"
}"##,
            ),
            Some(
                r##"{
    "matches": [
        {
            "id": "id:a4ayc_80_OEAAAAAAAAAXz",
            "is_deleted": false,
            "path": "/my_awesome/word.docx",
            "property_groups": [
                {
                    "fields": [
                        {
                            "name": "Security Policy",
                            "value": "Confidential"
                        }
                    ],
                    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
                }
            ]
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilePropertiesPropertiesUpdatePost => (
            Some(
                r##"{
            "path": "/my_awesome/word.docx",
            "property_groups": [
                {
                    "fields": [
                        {
                            "name": "Security Policy",
                            "value": "Confidential"
                        }
                    ],
                    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
                }
            ]
        }"##,
            ),
            None,
        ),
        Endpoint::SetProfilePhotoPost => todo!(),
        Endpoint::FilePropertiesTemplatesAddForUserPost => (
            Some(
                r##"{
    "description": "These properties describe how confidential this file or folder is.",
    "fields": [
        {
            "description": "This is the security policy of the file or folder described.\nPolicies can be Confidential, Public or Internal.",
            "name": "Security Policy",
            "type": "string"
        }
    ],
    "name": "Security"
}"##,
            ),
            Some(
                r##"{
    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
}"##,
            ),
        ),
        Endpoint::FilePropertiesTemplatesGetForUserPost => (
            Some(
                r##"{
    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
}"##,
            ),
            Some(
                r##"{
    "description": "These properties describe how confidential this file or folder is.",
    "fields": [
        {
            "description": "This is the security policy of the file or folder described.\nPolicies can be Confidential, Public or Internal.",
            "name": "Security Policy",
            "type": {
                ".tag": "string"
            }
        }
    ],
    "name": "Security"
}"##,
            ),
        ),
        Endpoint::FilePropertiesTemplatesListForUserPost => (
            None,
            Some(
                r##"{
    "template_ids": [
        "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
    ]
}"##,
            ),
        ),
        Endpoint::FilePropertiesTemplatesRemoveForUserPost => (
            Some(
                r##"{
    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
}"##,
            ),
            None,
        ),
        Endpoint::FilePropertiesTemplatesUpdateForUserPost => (
            Some(
                r##"{
            "path": "/my_awesome/word.docx",
            "property_groups": [
                {
                    "fields": [
                        {
                            "name": "Security Policy",
                            "value": "Confidential"
                        }
                    ],
                    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
                }
            ]
        }"##,
            ),
            None,
        ),
        Endpoint::FilePropertiesPropertiesUpdatePost => (
            Some(
                r##"{
    "path": "/my_awesome/word.docx",
    "update_property_groups": [
        {
            "add_or_update_fields": [
                {
                    "name": "Security Policy",
                    "value": "Confidential"
                }
            ],
            "remove_fields": [],
            "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
        }
    ]
}"##,
            ),
            None,
        ),
        Endpoint::TokenRevokePost => todo!(),
        Endpoint::UpdatePost => todo!(),
        Endpoint::CheckUserPost => todo!(),
        Endpoint::UsersFeaturesGetValuesPost => todo!(),
        Endpoint::UsersGetAccountBatchPost => todo!(),
        Endpoint::UsersGetAccountPost => todo!(),
        Endpoint::UsersGetCurrentAccountPost => todo!(),
        Endpoint::UsersGetSpaceUsagePost => todo!(),
    }
}

/// For testing purpose, it will replace original end-point with mock server url
fn test_url(url: &str) -> (String, Option<String>) {
    let idx = url.find("com").expect("should have com") + 3;
    let url = &url[idx..];
    let url_sync = format!(
        "http://{}:{}{}",
        MOCK_SERVER_SYNC_URL, MOCK_SERVER_SYNC_PORT, url
    );
    let url_async = format!(
        "http://{}:{}{}",
        MOCK_SERVER_ASYNC_URL, MOCK_SERVER_ASYNC_PORT, url
    );
    (url_sync, Some(url_async))
}

/// Enum representing necessary headers for requests
pub enum Headers {
    ContentTypeAppJson,
    TestAuthorization,
}

impl Headers {
    pub fn get_str(&self) -> (&str, &str) {
        match self {
            Headers::ContentTypeAppJson => ("Content-type", "application/json"),
            Headers::TestAuthorization => ("Authorization", "Bearer 123456"),
        }
    }
}

#[macro_export]
macro_rules! implement_utils {
    ($req_type:ty, $payload_type:ty) => {
        impl Utils<'_> for $req_type {
            type T = $payload_type;
            fn payload(&self) -> Option<&Self::T> {
                if self.payload.is_some() {
                    return Some(self.payload.as_ref().unwrap());
                }
                None
            }
        }
    };
}

use serde::{Deserialize, Serialize};

pub trait Utils<'a> {
    type T: Serialize + Deserialize<'a>;
    fn payload(&self) -> Option<&Self::T>;
}
