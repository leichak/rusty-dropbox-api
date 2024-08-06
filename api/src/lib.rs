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
const MOCK_SERVER_SYNC_PORT: u16 = 8002;
const MOCK_SERVER_ASYNC_URL: &str = "0.0.0.0";
const MOCK_SERVER_ASYNC_PORT: u16 = 1420;

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
                ..Default::default()
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
                    ..Default::default()
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
                let url = get_endpoint_url($endpoint).2.unwrap();
                println!("url {}", url);

                mock = server.mock("POST", url.as_str()).with_status(200);

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
                let url = get_endpoint_url($endpoint).2;

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
    ($req:ty, $resp:ident, $resp_payload:ty, $endpoints:expr, $headers:expr) => {
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

                println!("{:#?}", response);

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

                        let response: $resp_payload = serde_json::from_str(&text)
                            .map_err(|err| ApiError::ParsingError(err.into()))?;
                        let response = $resp { payload: response };
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

                    let response: $resp_payload = serde_json::from_str(&text)
                        .map_err(|err| ApiError::ParsingError(err.into()))?;
                    let response = $resp { payload: response };

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
    CheckAppUser,
    FileRequestsCountPost,
    FileRequestsCreatePost,
    FileRequestsDeleteAllClosedPost,
    ContactsDeleteManualContactsBatchPost,
    ContactsDeleteManualContactsPost,
    FileRequestsDeletePost,
    AddFolderMemberPost,
    FilesCopyPost,
    FilesCopyBatchPost,
    FilesCopyBatchCheckPost,
    FilesCopyReferenceGetPost,
    FilesCopyReferenceSavePost,
    FilesCreateFolderPost,
    FilesCreateFolderBatchPost,
    FilesCreateFolderBatchCheckPost,
    FilesDeletePost,
    FilesDeleteBatchPost,
    FilesDeleteBatchCheckPost,
    FilesDownloadPost,
    FilesDownloadZipPost,
    FilesExportPost,
    FilesGetFileLockBatchPost,
    FilesGetMetadataPost,
    FilesGetPreviewPost,
    FilesGetTemporaryLinkPost,
    FilesGetTemporaryUploadLinkPost,
    FilesGetThumbnailPost,
    FilesGetThumbnailBatchPost,
    FilesListFolderPost,
    FilesListFolderContinuePost,
    FilesListFolderGetLatestCursorPost,
    FilesListFolderLongpollPost,
    FilesListRevisionsPost,
    FilesLockFileBatchPost,
    FilesMovePost,
    FilesMoveBatchPost,
    FilesMoveBatchCheckPost,
    FilesPaperCreatePost,
    FilesPaperUpdatePost,
    FilesPermanentlyDeletePost,
    FilesRestorePost,
    FilesSaveUrlPost,
    FilesSaveUrlCheckJobStatusPost,
    FilesSearchPost,
    FilesSearchContinuePost,
    FilesTagsAddPost,
    FilesTagsGetPost,
    FilesTagsRemovePost,
    FilesUnlockFileBatchPost,
    FilesUploadPost,
    FilesUploadSessionAppendPost,
    FilesUploadSessionAppendBatchPost,
    FilesUploadSessionFinishPost,
    FilesUploadSessionFinishBatchPost,
    FilesUploadSessionFinishBatchCheckPost,
    FilesUploadSessionStartPost,
    FilesUploadSessionStartBatchPost,
    FileRequestsGetPost,
    FileRequestsListContinuePost,
    FileRequestsListPost,
    FilePropertiesPropertiesAddPost,
    FilePropertiesPropertiesOverwritePost,
    FilePropertiesPropertiesRemovePost,
    FilePropertiesPropertiesSearchContinuePost,
    FilePropertiesPropertiesSearchPost,
    FilePropertiesPropertiesUpdatePost,
    AccountSetProfilePhotoPost,
    OpenidUserInfoPost,
    FilePropertiesTemplatesAddForUserPost,
    FilePropertiesTemplatesGetForUserPost,
    FilePropertiesTemplatesListForUserPost,
    FilePropertiesTemplatesRemoveForUserPost,
    FilePropertiesTemplatesUpdateForUserPost,
    AuthTokenRevokePost,
    FileRequestsUpdatePost,
    CheckUserPost,
    SharingAddFileMemberPost,
    SharingAddFolderMemberPost,
    SharingCheckJobStatusPost,
    SharingCheckRemoveMemberJobStatusPost,
    SharingCheckShareJobStatusPost,
    SharingCreateSharedLinkWithSettingsPost,
    SharingGetFileMetadataPost,
    SharingGetFileMetadataBatchPost,
    SharingGetFolderMetadataPost,
    SharingGetSharedLinkFilePost,
    SharingGetSharedLinkMetadataPost,
    SharingListFileMembersPost,
    SharingListFileMembersBatchPost,
    SharingListFileMembersContinuePost,
    SharingListFolderMembersPost,
    SharingListFolderMembersContinuePost,
    SharingListFoldersPost,
    SharingListFoldersContinuePost,
    SharingListMountableFoldersPost,
    SharingListMountableFoldersContinuePost,
    SharingListReceivedFilesPost,
    SharingListReceivedFilesContinuePost,
    SharingListSharedLinksPost,
    SharingModifySharedLinksSettingsPost,
    SharingMountFolderPost,
    SharingRelinquishFileMembershipPost,
    SharingRelinquishFolderMembershipPost,
    SharingRemoveFileMember2Post,
    SharingRemoveFolderMemberPost,
    SharingRevokeSharedLinkPost,
    SharingSetAccessInheritancePost,
    SharingShareFolderPost,
    SharingTransferFolderPost,
    SharingUnmountFolderPost,
    SharingUnshareFilePost,
    SharingUnshareFolderPost,
    SharingUpdateFileMemberPost,
    SharingUpdateFolderMemberPost,
    SharingUpdateFolderPolicyPost,
    UsersFeaturesGetValuesPost,
    UsersGetAccountPost,
    UsersGetAccountBatchPost,
    UsersGetCurrentAccountPost,
    UsersGetSpaceUsagePost,
}

pub fn get_endpoint_url(endpoint: Endpoint) -> (String, Option<String>, Option<String>) {
    let url = match endpoint {
        Endpoint::AddFolderMemberPost => "https://api.dropboxapi.com/2/sharing/add_folder_member",
        Endpoint::CheckAppPost => "https://api.dropboxapi.com/2/check/app",
        Endpoint::FileRequestsCountPost => "https://api.dropboxapi.com/2/file_requests/count",
        Endpoint::FileRequestsCreatePost => "https://api.dropboxapi.com/2/file_requests/create",
        Endpoint::FileRequestsDeleteAllClosedPost => {
            "https://api.dropboxapi.com/2/delete_all_closed"
        }
        Endpoint::ContactsDeleteManualContactsBatchPost => {
            "https://api.dropboxapi.com/2/contacts/delete_manual_contacts_batch"
        }
        Endpoint::ContactsDeleteManualContactsPost => {
            "https://api.dropboxapi.com/2/contacts/delete_manual_contacts"
        }
        Endpoint::FilesDeletePost => "https://api.dropboxapi.com/2/delete",
        Endpoint::FilesUploadPost => "https://content.dropboxapi.com/2/files/upload",

        Endpoint::FilesTagsGetPost => "https://api.dropboxapi.com/2/get",
        Endpoint::FileRequestsListContinuePost => "https://api.dropboxapi.com/2/list/continue",
        Endpoint::FileRequestsListPost => "https://api.dropboxapi.com/2/list",
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
        Endpoint::AccountSetProfilePhotoPost => {
            "https://api.dropboxapi.com/2/account/set_profile_photo"
        }
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
        Endpoint::AuthTokenRevokePost => "https://api.dropboxapi.com/2/auth/token/revoke",
        Endpoint::FileRequestsUpdatePost => "https://api.dropboxapi.com/2/update",
        Endpoint::CheckUserPost => "https://api.dropboxapi.com/2/check/user",
        Endpoint::UsersFeaturesGetValuesPost => {
            "https://api.dropboxapi.com/2/users/features/get_values"
        }
        Endpoint::UsersGetAccountBatchPost => {
            "https://api.dropboxapi.com/2/users/get_account_batch"
        }
        Endpoint::FileRequestsGetPost => "",
        Endpoint::UsersGetAccountPost => "https://api.dropboxapi.com/2/users/get_account",
        Endpoint::UsersGetCurrentAccountPost => {
            "https://api.dropboxapi.com/2/users/get_current_account"
        }
        Endpoint::UsersGetSpaceUsagePost => "https://api.dropboxapi.com/2/users/get_space_usage",
        Endpoint::CheckAppUser => {
            "https://api.dropboxapi.com/2/check/app
"
        }
        Endpoint::FileRequestsDeletePost => {
            "https://api.dropboxapi.com/2/file_requests/delete
"
        }
        Endpoint::FilesCopyPost => {
            "https://api.dropboxapi.com/2/files/copy_v2
"
        }
        Endpoint::FilesCopyBatchPost => {
            "https://api.dropboxapi.com/2/files/copy_batch_v2
"
        }
        Endpoint::FilesCopyBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/copy_batch/check_v2
"
        }
        Endpoint::FilesCopyReferenceGetPost => {
            "https://api.dropboxapi.com/2/files/copy_reference/get
"
        }
        Endpoint::FilesCopyReferenceSavePost => {
            "https://api.dropboxapi.com/2/files/copy_reference/save
"
        }
        Endpoint::FilesCreateFolderPost => {
            "https://api.dropboxapi.com/2/files/create_folder_v2
"
        }
        Endpoint::FilesCreateFolderBatchPost => {
            "https://api.dropboxapi.com/2/files/create_folder_batch
"
        }
        Endpoint::FilesCreateFolderBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/create_folder_batch/check
"
        }
        Endpoint::FilesDeleteBatchPost => {
            "https://api.dropboxapi.com/2/files/delete_batch
"
        }
        Endpoint::FilesDeleteBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/delete_batch/check
"
        }
        Endpoint::FilesDownloadPost => "https://content.dropboxapi.com/2/files/download",
        Endpoint::FilesDownloadZipPost => {
            "https://content.dropboxapi.com/2/files/download_zip
"
        }
        Endpoint::FilesExportPost => {
            "https://content.dropboxapi.com/2/files/export
"
        }
        Endpoint::FilesGetFileLockBatchPost => {
            "https://api.dropboxapi.com/2/files/get_file_lock_batch
"
        }
        Endpoint::FilesGetMetadataPost => {
            "https://api.dropboxapi.com/2/files/get_metadata
"
        }
        Endpoint::FilesGetPreviewPost => {
            "https://content.dropboxapi.com/2/files/get_preview
"
        }
        Endpoint::FilesGetTemporaryLinkPost => {
            "https://api.dropboxapi.com/2/files/get_temporary_link
"
        }
        Endpoint::FilesGetTemporaryUploadLinkPost => {
            "https://api.dropboxapi.com/2/files/get_temporary_upload_link
"
        }
        Endpoint::FilesGetThumbnailPost => {
            "https://content.dropboxapi.com/2/files/get_thumbnail_v2"
        }
        Endpoint::FilesGetThumbnailBatchPost => {
            "https://content.dropboxapi.com/2/files/get_thumbnail_batch"
        }
        Endpoint::FilesListFolderPost => "https://api.dropboxapi.com/2/files/list_folder",
        Endpoint::FilesListFolderContinuePost => {
            "https://api.dropboxapi.com/2/files/list_folder/continue"
        }
        Endpoint::FilesListFolderGetLatestCursorPost => {
            "https://api.dropboxapi.com/2/files/list_folder/get_latest_cursor
"
        }
        Endpoint::FilesListFolderLongpollPost => {
            "https://notify.dropboxapi.com/2/files/list_folder/longpoll"
        }
        Endpoint::FilesListRevisionsPost => "https://api.dropboxapi.com/2/files/list_revisions",
        Endpoint::FilesLockFileBatchPost => "https://api.dropboxapi.com/2/files/lock_file_batch",
        Endpoint::FilesMovePost => "https://api.dropboxapi.com/2/files/move_v2",
        Endpoint::FilesMoveBatchPost => "https://api.dropboxapi.com/2/files/move_batch_v2",
        Endpoint::FilesMoveBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/move_batch/check_v2"
        }
        Endpoint::FilesPaperCreatePost => "https://api.dropboxapi.com/2/files/paper/create",
        Endpoint::FilesPaperUpdatePost => "https://api.dropboxapi.com/2/files/paper/update",
        Endpoint::FilesPermanentlyDeletePost => {
            "https://api.dropboxapi.com/2/files/permanently_delete"
        }
        Endpoint::FilesRestorePost => "https://api.dropboxapi.com/2/files/restore",
        Endpoint::FilesSaveUrlPost => "https://api.dropboxapi.com/2/files/save_url",
        Endpoint::FilesSaveUrlCheckJobStatusPost => {
            "https://api.dropboxapi.com/2/files/save_url/check_job_status"
        }
        Endpoint::FilesSearchPost => "https://api.dropboxapi.com/2/files/search_v2",
        Endpoint::FilesSearchContinuePost => {
            "https://api.dropboxapi.com/2/files/search/continue_v2"
        }
        Endpoint::FilesTagsAddPost => "https://api.dropboxapi.com/2/files/tags/add",
        Endpoint::FilesTagsGetPost => "https://api.dropboxapi.com/2/files/tags/get",
        Endpoint::FilesTagsRemovePost => "https://api.dropboxapi.com/2/files/tags/remove",
        Endpoint::FilesUnlockFileBatchPost => {
            "https://api.dropboxapi.com/2/files/unlock_file_batch"
        }
        Endpoint::FilesUploadPost => "https://content.dropboxapi.com/2/files/upload",
        Endpoint::FilesUploadSessionAppendPost => {
            "https://content.dropboxapi.com/2/files/upload_session/append_v2"
        }
        Endpoint::FilesUploadSessionAppendBatchPost => {
            "https://content.dropboxapi.com/2/files/upload_session/append_batch"
        }
        Endpoint::FilesUploadSessionFinishPost => {
            "https://content.dropboxapi.com/2/files/upload_session/finish"
        }
        Endpoint::FilesUploadSessionFinishBatchPost => {
            "https://api.dropboxapi.com/2/files/upload_session/finish_batch_v2"
        }
        Endpoint::FilesUploadSessionFinishBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/upload_session/finish_batch/check"
        }
        Endpoint::FilesUploadSessionStartPost => {
            "https://content.dropboxapi.com/2/files/upload_session/start"
        }
        Endpoint::FilesUploadSessionStartBatchPost => {
            "https://api.dropboxapi.com/2/files/upload_session/start_batch"
        }
        Endpoint::OpenidUserInfoPost => "https://api.dropboxapi.com/2/openid/userinfo",
        Endpoint::SharingAddFileMemberPost => {
            "https://api.dropboxapi.com/2/sharing/add_file_member"
        }
        Endpoint::SharingAddFolderMemberPost => {
            "https://api.dropboxapi.com/2/sharing/add_folder_member"
        }
        Endpoint::SharingCheckJobStatusPost => {
            "https://api.dropboxapi.com/2/sharing/check_job_status"
        }
        Endpoint::SharingCheckRemoveMemberJobStatusPost => {
            "https://api.dropboxapi.com/2/sharing/check_remove_member_job_status"
        }
        Endpoint::SharingCheckShareJobStatusPost => {
            "https://api.dropboxapi.com/2/sharing/check_share_job_status"
        }
        Endpoint::SharingCreateSharedLinkWithSettingsPost => {
            "https://api.dropboxapi.com/2/sharing/create_shared_link_with_settings"
        }
        Endpoint::SharingGetFileMetadataPost => {
            "https://api.dropboxapi.com/2/sharing/get_file_metadata"
        }
        Endpoint::SharingGetFileMetadataBatchPost => {
            "https://api.dropboxapi.com/2/sharing/get_file_metadata/batch"
        }
        Endpoint::SharingGetFolderMetadataPost => {
            "https://api.dropboxapi.com/2/sharing/get_folder_metadata"
        }
        Endpoint::SharingGetSharedLinkFilePost => {
            "https://content.dropboxapi.com/2/sharing/get_shared_link_file"
        }
        Endpoint::SharingGetSharedLinkMetadataPost => {
            "https://api.dropboxapi.com/2/sharing/get_shared_link_metadata"
        }
        Endpoint::SharingListFileMembersPost => {
            "https://api.dropboxapi.com/2/sharing/list_file_members"
        }
        Endpoint::SharingListFileMembersBatchPost => {
            "https://api.dropboxapi.com/2/sharing/list_file_members/batch"
        }
        Endpoint::SharingListFileMembersContinuePost => {
            "https://api.dropboxapi.com/2/sharing/list_file_members/continue"
        }
        Endpoint::SharingListFolderMembersPost => {
            "https://api.dropboxapi.com/2/sharing/list_folder_members"
        }
        Endpoint::SharingListFolderMembersContinuePost => {
            "https://api.dropboxapi.com/2/sharing/list_folder_members/continue"
        }
        Endpoint::SharingListFoldersPost => "https://api.dropboxapi.com/2/sharing/list_folders",
        Endpoint::SharingListFoldersContinuePost => {
            "https://api.dropboxapi.com/2/sharing/list_folders/continue"
        }
        Endpoint::SharingListMountableFoldersPost => {
            "https://api.dropboxapi.com/2/sharing/list_mountable_folders"
        }
        Endpoint::SharingListMountableFoldersContinuePost => {
            "https://api.dropboxapi.com/2/sharing/list_mountable_folders/continue"
        }
        Endpoint::SharingListReceivedFilesPost => {
            "https://api.dropboxapi.com/2/sharing/list_received_files"
        }
        Endpoint::SharingListReceivedFilesContinuePost => {
            "https://api.dropboxapi.com/2/sharing/list_received_files/continue"
        }
        Endpoint::SharingListSharedLinksPost => {
            "https://api.dropboxapi.com/2/sharing/list_shared_links"
        }
        Endpoint::SharingModifySharedLinksSettingsPost => {
            "https://api.dropboxapi.com/2/sharing/modify_shared_link_settings"
        }
        Endpoint::SharingMountFolderPost => "https://api.dropboxapi.com/2/sharing/mount_folder",
        Endpoint::SharingRelinquishFileMembershipPost => {
            "https://api.dropboxapi.com/2/sharing/relinquish_folder_membership"
        }
        Endpoint::SharingRelinquishFolderMembershipPost => {
            "https://api.dropboxapi.com/2/sharing/relinquish_folder_membership"
        }
        Endpoint::SharingRemoveFileMember2Post => {
            "https://api.dropboxapi.com/2/sharing/remove_file_member_2"
        }
        Endpoint::SharingRemoveFolderMemberPost => {
            "https://api.dropboxapi.com/2/sharing/remove_folder_member"
        }
        Endpoint::SharingRevokeSharedLinkPost => {
            "https://api.dropboxapi.com/2/sharing/revoke_shared_link"
        }
        Endpoint::SharingSetAccessInheritancePost => {
            "https://api.dropboxapi.com/2/sharing/set_access_inheritance"
        }
        Endpoint::SharingShareFolderPost => "https://api.dropboxapi.com/2/sharing/share_folder",
        Endpoint::SharingTransferFolderPost => {
            "https://api.dropboxapi.com/2/sharing/transfer_folder"
        }
        Endpoint::SharingUnmountFolderPost => "https://api.dropboxapi.com/2/sharing/unmount_folder",
        Endpoint::SharingUnshareFilePost => "https://api.dropboxapi.com/2/sharing/unshare_file",
        Endpoint::SharingUnshareFolderPost => "https://api.dropboxapi.com/2/sharing/unshare_folder",
        Endpoint::SharingUpdateFileMemberPost => {
            "https://api.dropboxapi.com/2/sharing/update_file_member"
        }
        Endpoint::SharingUpdateFolderMemberPost => {
            "https://api.dropboxapi.com/2/sharing/update_folder_member"
        }
        Endpoint::SharingUpdateFolderPolicyPost => {
            "https://api.dropboxapi.com/2/sharing/update_folder_policy
"
        }
    };

    let binding: (String, Option<String>, Option<String>) = (url.to_string(), None, None);
    #[cfg(feature = "test-utils")]
    let binding = test_url(url);

    binding
}

#[cfg(feature = "test-utils")]
pub fn get_endpoint_test_body_response(
    endpoint: Endpoint,
) -> (Option<&'static str>, Option<&'static str>) {
    match endpoint {
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
        Endpoint::AccountSetProfilePhotoPost => (
            Some(
                r##"{
    "photo": {
        ".tag": "base64_data",
        "base64_data": "SW1hZ2UgZGF0YSBpbiBiYXNlNjQtZW5jb2RlZCBieXRlcy4gTm90IGEgdmFsaWQgZXhhbXBsZS4="
    }
}"##,
            ),
            Some(
                r##"{
    "profile_photo_url": "https://dl-web.dropbox.com/account_photo/get/dbaphid%3AAAHWGmIXV3sUuOmBfTz0wPsiqHUpBWvv3ZA?vers=1556069330102&size=128x128"
}"##,
            ),
        ),
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
    "add_fields": [
        {
            "description": "This is the security policy of the file or folder described.\nPolicies can be Confidential, Public or Internal.",
            "name": "Security Policy",
            "type": "string"
        }
    ],
    "description": "These properties will describe how confidential this file or folder is.",
    "name": "New Security Template Name",
    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
}"##,
            ),
            Some(
                r##"{
    "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
}"##,
            ),
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
        Endpoint::CheckAppPost => "",
        Endpoint::CheckAppUser => "",
        Endpoint::FileRequestsCountPost => "",
        Endpoint::FileRequestsCreatePost => "",
        Endpoint::FileRequestsDeleteAllClosedPost => "",
        Endpoint::ContactsDeleteManualContactsBatchPost => "",
        Endpoint::ContactsDeleteManualContactsPost => "",
        Endpoint::FileRequestsDeletePost => "",
        Endpoint::AddFolderMemberPost => "",
        Endpoint::FilesCopyPost => "",
        Endpoint::FilesCopyBatchPost => "",
        Endpoint::FilesCopyBatchCheckPost => "",
        Endpoint::FilesCopyReferenceGetPost => "",
        Endpoint::FilesCopyReferenceSavePost => "",
        Endpoint::FilesCreateFolderPost => "",
        Endpoint::FilesCreateFolderBatchPost => "",
        Endpoint::FilesCreateFolderBatchCheckPost => "",
        Endpoint::FilesDeletePost => "",
        Endpoint::FilesDeleteBatchPost => "",
        Endpoint::FilesDeleteBatchCheckPost => "",
        Endpoint::FilesDownloadPost => "",
        Endpoint::FilesDownloadZipPost => "",
        Endpoint::FilesExportPost => "",
        Endpoint::FilesGetFileLockBatchPost => "",
        Endpoint::FilesGetMetadataPost => "",
        Endpoint::FilesGetPreviewPost => "",
        Endpoint::FilesGetTemporaryLinkPost => "",
        Endpoint::FilesGetTemporaryUploadLinkPost => "",
        Endpoint::FilesGetThumbnailPost => "",
        Endpoint::FilesGetThumbnailBatchPost => "",
        Endpoint::FilesListFolderPost => "",
        Endpoint::FilesListFolderContinuePost => "",
        Endpoint::FilesListFolderGetLatestCursorPost => "",
        Endpoint::FilesListFolderLongpollPost => "",
        Endpoint::FilesListRevisionsPost => "",
        Endpoint::FilesLockFileBatchPost => "",
        Endpoint::FilesMovePost => "",
        Endpoint::FilesMoveBatchPost => "",
        Endpoint::FilesMoveBatchCheckPost => "",
        Endpoint::FilesPaperCreatePost => "",
        Endpoint::FilesPaperUpdatePost => "",
        Endpoint::FilesPermanentlyDeletePost => "",
        Endpoint::FilesRestorePost => "",
        Endpoint::FilesSaveUrlPost => "",
        Endpoint::FilesSaveUrlCheckJobStatusPost => "",
        Endpoint::FilesSearchPost => "",
        Endpoint::FilesSearchContinuePost => "",
        Endpoint::FilesTagsAddPost => "",
        Endpoint::FilesTagsGetPost => "",
        Endpoint::FilesTagsRemovePost => "",
        Endpoint::FilesUnlockFileBatchPost => "",
        Endpoint::FilesUploadPost => "",
        Endpoint::FilesUploadSessionAppendPost => "",
        Endpoint::FilesUploadSessionAppendBatchPost => "",
        Endpoint::FilesUploadSessionFinishPost => "",
        Endpoint::FilesUploadSessionFinishBatchPost => "",
        Endpoint::FilesUploadSessionFinishBatchCheckPost => "",
        Endpoint::FilesUploadSessionStartPost => "",
        Endpoint::FilesUploadSessionStartBatchPost => "",
        Endpoint::FileRequestsGetPost => "",
        Endpoint::FileRequestsListContinuePost => "",
        Endpoint::FileRequestsListPost => "",
        Endpoint::OpenidUserInfoPost => "",
        Endpoint::AuthTokenRevokePost => "",
        Endpoint::FileRequestsUpdatePost => "",
        Endpoint::CheckUserPost => "",
        Endpoint::SharingAddFileMemberPost => "",
        Endpoint::SharingAddFolderMemberPost => "",
        Endpoint::SharingCheckJobStatusPost => "",
        Endpoint::SharingCheckRemoveMemberJobStatusPost => "",
        Endpoint::SharingCheckShareJobStatusPost => "",
        Endpoint::SharingCreateSharedLinkWithSettingsPost => "",
        Endpoint::SharingGetFileMetadataPost => "",
        Endpoint::SharingGetFileMetadataBatchPost => "",
        Endpoint::SharingGetFolderMetadataPost => "",
        Endpoint::SharingGetSharedLinkFilePost => "",
        Endpoint::SharingGetSharedLinkMetadataPost => "",
        Endpoint::SharingListFileMembersPost => "",
        Endpoint::SharingListFileMembersBatchPost => "",
        Endpoint::SharingListFileMembersContinuePost => "",
        Endpoint::SharingListFolderMembersPost => "",
        Endpoint::SharingListFolderMembersContinuePost => "",
        Endpoint::SharingListFoldersPost => "",
        Endpoint::SharingListFoldersContinuePost => "",
        Endpoint::SharingListMountableFoldersPost => "",
        Endpoint::SharingListMountableFoldersContinuePost => "",
        Endpoint::SharingListReceivedFilesPost => "",
        Endpoint::SharingListReceivedFilesContinuePost => "",
        Endpoint::SharingListSharedLinksPost => "",
        Endpoint::SharingModifySharedLinksSettingsPost => "",
        Endpoint::SharingMountFolderPost => "",
        Endpoint::SharingRelinquishFileMembershipPost => "",
        Endpoint::SharingRelinquishFolderMembershipPost => "",
        Endpoint::SharingRemoveFileMember2Post => "",
        Endpoint::SharingRemoveFolderMemberPost => "",
        Endpoint::SharingRevokeSharedLinkPost => "",
        Endpoint::SharingSetAccessInheritancePost => "",
        Endpoint::SharingShareFolderPost => "",
        Endpoint::SharingTransferFolderPost => "",
        Endpoint::SharingUnmountFolderPost => "",
        Endpoint::SharingUnshareFilePost => "",
        Endpoint::SharingUnshareFolderPost => "",
        Endpoint::SharingUpdateFileMemberPost => "",
        Endpoint::SharingUpdateFolderMemberPost => "",
        Endpoint::SharingUpdateFolderPolicyPost => "",
        Endpoint::UsersFeaturesGetValuesPost => "",
        Endpoint::UsersGetAccountPost => "",
        Endpoint::UsersGetAccountBatchPost => "",
        Endpoint::UsersGetCurrentAccountPost => "",
        Endpoint::UsersGetSpaceUsagePost => "",
    }
}

/// For testing purpose, it will replace original end-point with mock server url
fn test_url(url: &str) -> (String, Option<String>, Option<String>) {
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
    (url_sync, Some(url_async), Some(url.to_string()))
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
