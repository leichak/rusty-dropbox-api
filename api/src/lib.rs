use lazy_static::lazy_static;
use thiserror::Error;

/// Async and sync clients
lazy_static! {
    pub static ref SyncClient: reqwest::blocking::Client = reqwest::blocking::Client::new();
    pub static ref AsyncClient: reqwest::Client = reqwest::Client::new();
}

pub use {anyhow, anyhow::Result, async_trait, futures::future::BoxFuture, reqwest, serde_json};

/// Enum describing set of errors that can occur possibly
/// Thiserror macro to derive std::error::Error trait
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unknown")] // display trait
    Unknown,
    #[error("Reqwest error {0}")] // display trait
    RequestError(Box<dyn std::error::Error>),
    #[error("Parsing error {0}")] // display trait
    ParsingError(Box<dyn std::error::Error>),
    #[error("Dropbox error {0}")] // display trait
    DropBoxError(Box<dyn std::error::Error>),
}

// /// Enum that stores optional input to
// pub enum CallType<'a, O: Sized, E: Sized, F: ?Sized> {
//     Sync(Option<Result<O, E>>),
//     Async(Option<Result<&'a mut F, E>>),
// }

// /// Trait bundling async and sync requests
// /// It has call that takes CallType and returns Call type that stores Result
// /// The Result can store Output or can store Error
// pub trait Api<O: Sized, E: Sized, F: ?Sized> {
//     fn call(&self, call_type: CallType<O, E, F>) -> CallType<O, E, F>;
// }

// Trait for sync
pub trait SyncService<O: Sized, E: Sized> {
    fn call(&self) -> Result<O, E>;
}

// Trait for async
pub trait AsyncService<O: Sized, E: Sized, F: Sized> {
    fn call(&self) -> Result<F, E>;
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

impl Endpoint {
    /// Function returning appropriate endpoints
    /// to do files, sharing
    pub fn get_endpoint_url(&self) -> &'static str {
        match &self {
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
            Endpoint::SetProfilePhotoPost => {
                "https://api.dropboxapi.com/2/account/set_profile_photo"
            }
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
            Endpoint::UsersGetSpaceUsagePost => {
                "https://api.dropboxapi.com/2/users/get_space_usage"
            }
        }
    }
}

pub enum Headers {
    ContentTypeAppJson,
}

impl Headers {
    pub fn get_str(&self) -> (&str, &str) {
        match self {
            Headers::ContentTypeAppJson => ("Content-type", "application/json"),
        }
    }
}
