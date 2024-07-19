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
const MOCK_SERVER_SYNC_PORT: u16 = 1234;
const MOCK_SERVER_ASYNC_URL: &str = "0.0.0.0";
const MOCK_SERVER_ASYNC_PORT: u16 = 123;

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
    if let Some(server) = MOCK_SERVER_ASYNC.get() {
        return server.lock().expect("Failed to lock");
    }

    let server = Mutex::new(
        mockito::Server::new_with_opts_async(mockito::ServerOpts {
            host: MOCK_SERVER_ASYNC_URL,
            port: MOCK_SERVER_ASYNC_PORT,
            assert_on_drop: false,
        })
        .await,
    );

    MOCK_SERVER_ASYNC
        .get_or_init(|| server)
        .lock()
        .expect("Failed to lock")
}

/// Enum describing set of errors that can occur possibly
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
/// Async call will return future that needs to be awaited using own executor
/// Sync will block and return result
pub trait Service<O: Sized, F: Sized> {
    fn call_sync(&self) -> Result<Option<O>>;
    fn call(&self) -> Result<F>;
}

/// Enum representing api available endpoints
/// It is passed to fhe function
#[derive(Debug)]
pub enum Endpoint {
    AppPost,
    CountPost,
    CreatePost,
    DeleteAllClosedPost,
    DeleteManualContactsBatchPost,
    DeleteManualContactsPost,
    DeletePost,
    FilesDownloadPost,
    FilesUploadPost,
    FilesUploadSessionAppendV2Post,
    FilesUploadSessionFinishPost,
    FilesUploadSessionStartPost,
    GetPost,
    ListContinuePost,
    ListPost,
    PropertiesAddPost,
    PropertiesOverwritePost,
    PropertiesRemovePost,
    PropertiesSearchContinuePost,
    PropertiesSearchPost,
    PropertiesUpdatePost,
    SetProfilePhotoPost,
    TemplatesAddForUserPost,
    TemplatesGetForUserPost,
    TemplatesListForUserPost,
    TemplatesRemoveForUserPost,
    TemplatesUpdateForUserPost,
    TokenRevokePost,
    UpdatePost,
    UserPost,
    UsersFeaturesGetValuesPost,
    UsersGetAccountBatchPost,
    UsersGetAccountPost,
    UsersGetCurrentAccountPost,
    UsersGetSpaceUsagePost,
}

pub fn get_endpoint_url(endpoint: Endpoint) -> (String, Option<String>) {
    let url = match endpoint {
        Endpoint::AppPost => "https://api.dropboxapi.com/2/check/app",
        Endpoint::CountPost => "https://api.dropboxapi.com/2/file_requests/count",
        Endpoint::CreatePost => "https://api.dropboxapi.com/2/file_requests/create",
        Endpoint::DeleteAllClosedPost => "https://api.dropboxapi.com/2/delete_all_closed",
        Endpoint::DeleteManualContactsBatchPost => {
            "https://api.dropboxapi.com/2/contacts/delete_manual_contacts_batch"
        }
        Endpoint::DeleteManualContactsPost => {
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
        Endpoint::PropertiesAddPost => {
            "https://api.dropboxapi.com/2/file_properties/properties/add"
        }
        Endpoint::PropertiesOverwritePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/overwrite"
        }
        Endpoint::PropertiesRemovePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/remove"
        }
        Endpoint::PropertiesSearchContinuePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/search/continue"
        }
        Endpoint::PropertiesSearchPost => {
            "https://api.dropboxapi.com/2/file_properties/properties/search"
        }
        Endpoint::PropertiesUpdatePost => {
            "https://api.dropboxapi.com/2/file_properties/properties/update"
        }
        Endpoint::SetProfilePhotoPost => "https://api.dropboxapi.com/2/account/set_profile_photo",
        Endpoint::TemplatesAddForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/add_for_user"
        }
        Endpoint::TemplatesGetForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/get_for_user"
        }
        Endpoint::TemplatesListForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/list_for_user"
        }
        Endpoint::TemplatesRemoveForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/remove_for_user"
        }
        Endpoint::TemplatesUpdateForUserPost => {
            "https://api.dropboxapi.com/2/file_properties/templates/update_for_user"
        }
        Endpoint::TokenRevokePost => "https://api.dropboxapi.com/2/auth/token/revoke",
        Endpoint::UpdatePost => "https://api.dropboxapi.com/2/update",
        Endpoint::UserPost => "https://api.dropboxapi.com/2/check/user",
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

pub enum Headers {
    ContentTypeAppJson,
    Authorization,
}

impl Headers {
    pub fn get_str(&self) -> (&str, &str) {
        match self {
            Headers::ContentTypeAppJson => ("Content-type", "application/json"),
            Headers::Authorization => ("Authorization", "Bearer 123456"),
        }
    }
}
