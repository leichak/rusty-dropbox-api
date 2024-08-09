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
    Request(anyhow::Error),
    #[error("Parsing error {0}")] // display trait
    Parsing(anyhow::Error),
    #[error("Dropbox error {0}")] // display trait
    DropBox(anyhow::Error),
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
                    .map_err(|err| ApiError::Request(err.into()))?;

                match response.error_for_status() {
                    Ok(response) => {
                        let text = response
                            .text()
                            .map_err(|err| ApiError::Parsing(err.into()))?;

                        if text.is_empty() {
                            return Ok(None);
                        }

                        let response: $resp_payload = serde_json::from_str(&text)
                            .map_err(|err| ApiError::Parsing(err.into()))?;
                        let response = $resp { payload: response };
                        Ok(Some(response))
                    }
                    Err(err) => Err(ApiError::DropBox(err.into()).into()),
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
                        .map_err(|err| ApiError::Request(err.into()))?;

                    let response = response
                        .error_for_status()
                        .map_err(|err| ApiError::DropBox(err.into()))?;

                    let text = response
                        .text()
                        .await
                        .map_err(|err| ApiError::Parsing(err.into()))?;

                    if text.is_empty() {
                        return Ok(None);
                    }

                    let response: $resp_payload =
                        serde_json::from_str(&text).map_err(|err| ApiError::Parsing(err.into()))?;
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
        Endpoint::CheckAppPost => "https://api.dropboxapi.com/2/check/app",
        Endpoint::FileRequestsCountPost => "https://api.dropboxapi.com/2/file_requests/count",
        Endpoint::FileRequestsCreatePost => "https://api.dropboxapi.com/2/file_requests/create",
        Endpoint::FileRequestsGetPost => "https://api.dropboxapi.com/2/file_requests/get",
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
        Endpoint::CheckAppPost => (
            Some(
                r##"{
    "query": "foo"
}"##,
            ),
            Some(
                r##"{
    "result": "foo"
}"##,
            ),
        ),
        Endpoint::CheckAppUser => (
            Some(
                r##"{
    "query": "foo"
}
"##,
            ),
            Some(
                r##"{
    "result": "foo"
}"##,
            ),
        ),
        Endpoint::FileRequestsGetPost => (
            Some(
                r##"{
    "id": "oaCAVmEyrqYnkZX9955Y"
}"##,
            ),
            Some(
                r##"{
    "created": "2015-10-05T17:00:00Z",
    "deadline": {
        "allow_late_uploads": {
            ".tag": "seven_days"
        },
        "deadline": "2020-10-12T17:00:00Z"
    },
    "description": "Please submit your homework here.",
    "destination": "/File Requests/Homework",
    "file_count": 3,
    "id": "oaCAVmEyrqYnkZX9955Y",
    "is_open": true,
    "title": "Homework submission",
    "url": "https://www.dropbox.com/request/oaCAVmEyrqYnkZX9955Y"
}"##,
            ),
        ),

        Endpoint::FileRequestsDeletePost => (
            Some(
                r##"{
    "ids": [
        "oaCAVmEyrqYnkZX9955Y",
        "BaZmehYoXMPtaRmfTbSG"
    ]
}"##,
            ),
            Some(
                r##"{
    "file_requests": [
        {
            "created": "2015-10-05T17:00:00Z",
            "deadline": {
                "allow_late_uploads": {
                    ".tag": "seven_days"
                },
                "deadline": "2020-10-12T17:00:00Z"
            },
            "description": "Please submit your homework here.",
            "destination": "/File Requests/Homework",
            "file_count": 3,
            "id": "oaCAVmEyrqYnkZX9955Y",
            "is_open": true,
            "title": "Homework submission",
            "url": "https://www.dropbox.com/request/oaCAVmEyrqYnkZX9955Y"
        },
        {
            "created": "2015-11-02T04:00:00Z",
            "deadline": {
                "deadline": "2020-10-12T17:00:00Z"
            },
            "destination": "/Photo contest entries",
            "file_count": 105,
            "id": "BAJ7IrRGicQKGToykQdB",
            "is_open": true,
            "title": "Photo contest submission",
            "url": "https://www.dropbox.com/request/BAJ7IrRGjcQKGToykQdB"
        },
        {
            "created": "2015-12-15T13:02:00Z",
            "destination": "/Wedding photos",
            "file_count": 37,
            "id": "rxwMPvK3ATTa0VxOJu5T",
            "is_open": true,
            "title": "Wedding photo submission",
            "url": "https://www.dropbox.com/request/rxwMPvK3ATTa0VxOJu5T"
        }
    ]
}"##,
            ),
        ),

        Endpoint::FileRequestsCountPost => (
            None,
            Some(
                r##"{
    "file_request_count": 15
}"##,
            ),
        ),
        Endpoint::FileRequestsCreatePost => (
            Some(
                r##"{
    "deadline": {
        "allow_late_uploads": "seven_days",
        "deadline": "2020-10-12T17:00:00Z"
    },
    "destination": "/File Requests/Homework",
    "open": true,
    "title": "Homework submission"
}"##,
            ),
            Some(
                r##"{
    "created": "2015-10-05T17:00:00Z",
    "deadline": {
        "allow_late_uploads": {
            ".tag": "seven_days"
        },
        "deadline": "2020-10-12T17:00:00Z"
    },
    "description": "Please submit your homework here.",
    "destination": "/File Requests/Homework",
    "file_count": 3,
    "id": "oaCAVmEyrqYnkZX9955Y",
    "is_open": true,
    "title": "Homework submission",
    "url": "https://www.dropbox.com/request/oaCAVmEyrqYnkZX9955Y"
}"##,
            ),
        ),
        Endpoint::FileRequestsDeleteAllClosedPost => (
            None,
            Some(
                r##"{
    "file_requests": [
        {
            "created": "2015-10-05T17:00:00Z",
            "deadline": {
                "allow_late_uploads": {
                    ".tag": "seven_days"
                },
                "deadline": "2020-10-12T17:00:00Z"
            },
            "description": "Please submit your homework here.",
            "destination": "/File Requests/Homework",
            "file_count": 3,
            "id": "oaCAVmEyrqYnkZX9955Y",
            "is_open": true,
            "title": "Homework submission",
            "url": "https://www.dropbox.com/request/oaCAVmEyrqYnkZX9955Y"
        },
        {
            "created": "2015-11-02T04:00:00Z",
            "deadline": {
                "deadline": "2020-10-12T17:00:00Z"
            },
            "destination": "/Photo contest entries",
            "file_count": 105,
            "id": "BAJ7IrRGicQKGToykQdB",
            "is_open": true,
            "title": "Photo contest submission",
            "url": "https://www.dropbox.com/request/BAJ7IrRGjcQKGToykQdB"
        },
        {
            "created": "2015-12-15T13:02:00Z",
            "destination": "/Wedding photos",
            "file_count": 37,
            "id": "rxwMPvK3ATTa0VxOJu5T",
            "is_open": true,
            "title": "Wedding photo submission",
            "url": "https://www.dropbox.com/request/rxwMPvK3ATTa0VxOJu5T"
        }
    ]
}"##,
            ),
        ),
        Endpoint::FileRequestsListPost => (
            Some(
                r##"{
    "limit": 1000
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "file_requests": [
        {
            "created": "2015-10-05T17:00:00Z",
            "deadline": {
                "allow_late_uploads": {
                    ".tag": "seven_days"
                },
                "deadline": "2020-10-12T17:00:00Z"
            },
            "description": "Please submit your homework here.",
            "destination": "/File Requests/Homework",
            "file_count": 3,
            "id": "oaCAVmEyrqYnkZX9955Y",
            "is_open": true,
            "title": "Homework submission",
            "url": "https://www.dropbox.com/request/oaCAVmEyrqYnkZX9955Y"
        },
        {
            "created": "2015-11-02T04:00:00Z",
            "deadline": {
                "deadline": "2020-10-12T17:00:00Z"
            },
            "destination": "/Photo contest entries",
            "file_count": 105,
            "id": "BAJ7IrRGicQKGToykQdB",
            "is_open": true,
            "title": "Photo contest submission",
            "url": "https://www.dropbox.com/request/BAJ7IrRGjcQKGToykQdB"
        },
        {
            "created": "2015-12-15T13:02:00Z",
            "destination": "/Wedding photos",
            "file_count": 37,
            "id": "rxwMPvK3ATTa0VxOJu5T",
            "is_open": true,
            "title": "Wedding photo submission",
            "url": "https://www.dropbox.com/request/rxwMPvK3ATTa0VxOJu5T"
        }
    ],
    "has_more": true
}"##,
            ),
        ),
        Endpoint::FileRequestsListContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "file_requests": [
        {
            "created": "2015-10-05T17:00:00Z",
            "deadline": {
                "allow_late_uploads": {
                    ".tag": "seven_days"
                },
                "deadline": "2020-10-12T17:00:00Z"
            },
            "description": "Please submit your homework here.",
            "destination": "/File Requests/Homework",
            "file_count": 3,
            "id": "oaCAVmEyrqYnkZX9955Y",
            "is_open": true,
            "title": "Homework submission",
            "url": "https://www.dropbox.com/request/oaCAVmEyrqYnkZX9955Y"
        },
        {
            "created": "2015-11-02T04:00:00Z",
            "deadline": {
                "deadline": "2020-10-12T17:00:00Z"
            },
            "destination": "/Photo contest entries",
            "file_count": 105,
            "id": "BAJ7IrRGicQKGToykQdB",
            "is_open": true,
            "title": "Photo contest submission",
            "url": "https://www.dropbox.com/request/BAJ7IrRGjcQKGToykQdB"
        },
        {
            "created": "2015-12-15T13:02:00Z",
            "destination": "/Wedding photos",
            "file_count": 37,
            "id": "rxwMPvK3ATTa0VxOJu5T",
            "is_open": true,
            "title": "Wedding photo submission",
            "url": "https://www.dropbox.com/request/rxwMPvK3ATTa0VxOJu5T"
        }
    ],
    "has_more": true
}"##,
            ),
        ),
        Endpoint::FileRequestsUpdatePost => (
            Some(
                r##"{
    "deadline": {
        ".tag": "update",
        "allow_late_uploads": "seven_days",
        "deadline": "2020-10-12T17:00:00Z"
    },
    "destination": "/File Requests/Homework",
    "id": "oaCAVmEyrqYnkZX9955Y",
    "open": true,
    "title": "Homework submission"
}"##,
            ),
            Some(
                r##"{
    "created": "2015-10-05T17:00:00Z",
    "deadline": {
        "allow_late_uploads": {
            ".tag": "seven_days"
        },
        "deadline": "2020-10-12T17:00:00Z"
    },
    "description": "Please submit your homework here.",
    "destination": "/File Requests/Homework",
    "file_count": 3,
    "id": "oaCAVmEyrqYnkZX9955Y",
    "is_open": true,
    "title": "Homework submission",
    "url": "https://www.dropbox.com/request/oaCAVmEyrqYnkZX9955Y"
}"##,
            ),
        ),
        Endpoint::ContactsDeleteManualContactsBatchPost => (None, None),
        Endpoint::ContactsDeleteManualContactsPost => (None, None),
        Endpoint::FilesCopyPost => (
            Some(
                r##"{
    "allow_ownership_transfer": false,
    "allow_shared_folder": false,
    "autorename": false,
    "from_path": "/Homework/math",
    "to_path": "/Homework/algebra"
}"##,
            ),
            Some(
                r##"{
    "metadata": {
        ".tag": "file",
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}
"##,
            ),
        ),
        Endpoint::FilesCopyBatchPost => (
            Some(
                r##"{
    "autorename": false,
    "entries": [
        {
            "from_path": "/Homework/math",
            "to_path": "/Homework/algebra"
        }
    ]
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "success": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}
"##,
            ),
        ),
        Endpoint::FilesCopyBatchCheckPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}
"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "success": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesCopyReferenceGetPost => (
            Some(
                r##"{
    "path": "/video.mp4"
}"##,
            ),
            Some(
                r##"{
    "copy_reference": "z1X6ATl6aWtzOGq0c3g5Ng",
    "expires": "2045-05-12T15:50:38Z",
    "metadata": {
        ".tag": "file",
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}"##,
            ),
        ),
        Endpoint::FilesCopyReferenceSavePost => (
            Some(
                r##"{
    "copy_reference": "z1X6ATl6aWtzOGq0c3g5Ng",
    "path": "/video.mp4"
}"##,
            ),
            Some(
                r##"{
    "metadata": {
        ".tag": "file",
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}"##,
            ),
        ),
        Endpoint::FilesCreateFolderPost => (
            Some(
                r##"{
    "autorename": false,
    "path": "/Homework/math"
}"##,
            ),
            Some(
                r##"{
    "metadata": {
        "id": "id:a4ayc_80_OEAAAAAAAAAXz",
        "name": "math",
        "path_display": "/Homework/math",
        "path_lower": "/homework/math",
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
        ],
        "sharing_info": {
            "no_access": false,
            "parent_shared_folder_id": "84528192421",
            "read_only": false,
            "traverse_only": false
        }
    }
}"##,
            ),
        ),
        Endpoint::FilesCreateFolderBatchPost => (
            Some(
                r##"{
    "autorename": false,
    "force_async": false,
    "paths": [
        "/Homework/math"
    ]
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "metadata": {
                "id": "id:a4ayc_80_OEAAAAAAAAAXz",
                "name": "math",
                "path_display": "/Homework/math",
                "path_lower": "/homework/math",
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
                ],
                "sharing_info": {
                    "no_access": false,
                    "parent_shared_folder_id": "84528192421",
                    "read_only": false,
                    "traverse_only": false
                }
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesCreateFolderBatchCheckPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "metadata": {
                "id": "id:a4ayc_80_OEAAAAAAAAAXz",
                "name": "math",
                "path_display": "/Homework/math",
                "path_lower": "/homework/math",
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
                ],
                "sharing_info": {
                    "no_access": false,
                    "parent_shared_folder_id": "84528192421",
                    "read_only": false,
                    "traverse_only": false
                }
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesDeletePost => (
            Some(
                r##"{
    "path": "/Homework/math/Prime_Numbers.txt"
}"##,
            ),
            Some(
                r##"{
    "metadata": {
        ".tag": "file",
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}"##,
            ),
        ),
        Endpoint::FilesDeleteBatchPost => (
            Some(
                r##"{
    "entries": [
        {
            "path": "/Homework/math/Prime_Numbers.txt"
        }
    ]
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "metadata": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesDeleteBatchCheckPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "metadata": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesDownloadPost => (
            Some(
                r##"{
    "path": "/Homework/math/Prime_Numbers.txt"
}"##,
            ),
            Some(
                r##"{
    "client_modified": "2015-05-12T15:50:38Z",
    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "file_lock_info": {
        "created": "2015-05-12T15:50:38Z",
        "is_lockholder": true,
        "lockholder_name": "Imaginary User"
    },
    "has_explicit_shared_members": false,
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_downloadable": true,
    "name": "Prime_Numbers.txt",
    "path_display": "/Homework/math/Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
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
    ],
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "sharing_info": {
        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
        "parent_shared_folder_id": "84528192421",
        "read_only": true
    },
    "size": 7212
}"##,
            ),
        ),
        Endpoint::FilesDownloadZipPost => (
            Some(
                r##"{
    "path": "rev:a1c10ce0dd78"
}"##,
            ),
            Some(
                r##"{
    "metadata": {
        "id": "id:a4ayc_80_OEAAAAAAAAAXz",
        "name": "math",
        "path_display": "/Homework/math",
        "path_lower": "/homework/math",
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
        ],
        "sharing_info": {
            "no_access": false,
            "parent_shared_folder_id": "84528192421",
            "read_only": false,
            "traverse_only": false
        }
    }
}"##,
            ),
        ),
        Endpoint::FilesExportPost => (
            Some(
                r##"{
    "path": "id:a4ayc_80_OEAAAAAAAAAYa"
}"##,
            ),
            Some(
                r##"{
    "export_metadata": {
        "export_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "name": "Prime_Numbers.xlsx",
        "size": 7189
    },
    "file_metadata": {
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}
"##,
            ),
        ),
        Endpoint::FilesGetFileLockBatchPost => (
            Some(
                r##"{
    "entries": [
        {
            "path": "/John Doe/sample/test.pdf"
        }
    ]
}"##,
            ),
            Some(
                r##"S
{
    "entries": [
        {
            ".tag": "success",
            "lock": {
                "content": {
                    ".tag": "single_user",
                    "created": "2015-05-12T15:50:38Z",
                    "lock_holder_account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "lock_holder_team_id": "dbtid:1234abcd"
                }
            },
            "metadata": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesGetMetadataPost => (
            Some(
                r##"{
    "include_deleted": false,
    "include_has_explicit_shared_members": false,
    "include_media_info": false,
    "path": "/Homework/math"
}"##,
            ),
            Some(
                r##"{
    ".tag": "file",
    "client_modified": "2015-05-12T15:50:38Z",
    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "file_lock_info": {
        "created": "2015-05-12T15:50:38Z",
        "is_lockholder": true,
        "lockholder_name": "Imaginary User"
    },
    "has_explicit_shared_members": false,
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_downloadable": true,
    "name": "Prime_Numbers.txt",
    "path_display": "/Homework/math/Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
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
    ],
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "sharing_info": {
        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
        "parent_shared_folder_id": "84528192421",
        "read_only": true
    },
    "size": 7212
}"##,
            ),
        ),
        Endpoint::FilesGetPreviewPost => (
            Some(
                r##"{
    "path": "/word.docx"
}"##,
            ),
            Some(
                r##"{
    "client_modified": "2015-05-12T15:50:38Z",
    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "file_lock_info": {
        "created": "2015-05-12T15:50:38Z",
        "is_lockholder": true,
        "lockholder_name": "Imaginary User"
    },
    "has_explicit_shared_members": false,
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_downloadable": true,
    "name": "Prime_Numbers.txt",
    "path_display": "/Homework/math/Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
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
    ],
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "sharing_info": {
        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
        "parent_shared_folder_id": "84528192421",
        "read_only": true
    },
    "size": 7212
}"##,
            ),
        ),
        Endpoint::FilesGetTemporaryLinkPost => (
            Some(
                r##"{
    "path": "/video.mp4"
}"##,
            ),
            Some(
                r##"{
    "link": "https://ucabc123456.dl.dropboxusercontent.com/cd/0/get/abcdefghijklmonpqrstuvwxyz1234567890/file",
    "metadata": {
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}"##,
            ),
        ),
        Endpoint::FilesGetTemporaryUploadLinkPost => (
            Some(
                r##"{
    "commit_info": {
        "autorename": true,
        "mode": "add",
        "mute": false,
        "path": "/Homework/math/Matrices.txt",
        "strict_conflict": false
    },
    "duration": 3600
}"##,
            ),
            Some(
                r##"{
    "link": "https://content.dropboxapi.com/apitul/1/bNi2uIYF51cVBND"
}"##,
            ),
        ),
        Endpoint::FilesGetThumbnailPost => (
            Some(
                r##"{
    "format": "jpeg",
    "mode": "strict",
    "quality": "quality_80",
    "resource": {
        ".tag": "path",
        "path": "/a.docx"
    },
    "size": "w64h64"
}"##,
            ),
            Some(
                r##"{
    "file_metadata": {
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}"##,
            ),
        ),
        Endpoint::FilesGetThumbnailBatchPost => (
            Some(
                r##"{
    "entries": [
        {
            "format": "jpeg",
            "mode": "strict",
            "path": "/image.jpg",
            "quality": "quality_80",
            "size": "w64h64"
        }
    ]
}"##,
            ),
            Some(
                r##"{
    "entries": [
        {
            ".tag": "success",
            "metadata": {
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            },
            "thumbnail": "iVBORw0KGgoAAAANSUhEUgAAAdcAAABrCAMAAAI="
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesListFolderPost => (
            Some(
                r##"{
    "include_deleted": false,
    "include_has_explicit_shared_members": false,
    "include_media_info": false,
    "include_mounted_folders": true,
    "include_non_downloadable_files": true,
    "path": "/Homework/math",
    "recursive": false
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "entries": [
        {
            ".tag": "file",
            "client_modified": "2015-05-12T15:50:38Z",
            "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "file_lock_info": {
                "created": "2015-05-12T15:50:38Z",
                "is_lockholder": true,
                "lockholder_name": "Imaginary User"
            },
            "has_explicit_shared_members": false,
            "id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_downloadable": true,
            "name": "Prime_Numbers.txt",
            "path_display": "/Homework/math/Prime_Numbers.txt",
            "path_lower": "/homework/math/prime_numbers.txt",
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
            ],
            "rev": "a1c10ce0dd78",
            "server_modified": "2015-05-12T15:50:38Z",
            "sharing_info": {
                "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "parent_shared_folder_id": "84528192421",
                "read_only": true
            },
            "size": 7212
        },
        {
            ".tag": "folder",
            "id": "id:a4ayc_80_OEAAAAAAAAAXz",
            "name": "math",
            "path_display": "/Homework/math",
            "path_lower": "/homework/math",
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
            ],
            "sharing_info": {
                "no_access": false,
                "parent_shared_folder_id": "84528192421",
                "read_only": false,
                "traverse_only": false
            }
        }
    ],
    "has_more": false
}"##,
            ),
        ),
        Endpoint::FilesListFolderContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "entries": [
        {
            ".tag": "file",
            "client_modified": "2015-05-12T15:50:38Z",
            "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "file_lock_info": {
                "created": "2015-05-12T15:50:38Z",
                "is_lockholder": true,
                "lockholder_name": "Imaginary User"
            },
            "has_explicit_shared_members": false,
            "id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_downloadable": true,
            "name": "Prime_Numbers.txt",
            "path_display": "/Homework/math/Prime_Numbers.txt",
            "path_lower": "/homework/math/prime_numbers.txt",
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
            ],
            "rev": "a1c10ce0dd78",
            "server_modified": "2015-05-12T15:50:38Z",
            "sharing_info": {
                "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "parent_shared_folder_id": "84528192421",
                "read_only": true
            },
            "size": 7212
        },
        {
            ".tag": "folder",
            "id": "id:a4ayc_80_OEAAAAAAAAAXz",
            "name": "math",
            "path_display": "/Homework/math",
            "path_lower": "/homework/math",
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
            ],
            "sharing_info": {
                "no_access": false,
                "parent_shared_folder_id": "84528192421",
                "read_only": false,
                "traverse_only": false
            }
        }
    ],
    "has_more": false
}"##,
            ),
        ),
        Endpoint::FilesListFolderGetLatestCursorPost => (
            Some(
                r##"{
    "include_deleted": false,
    "include_has_explicit_shared_members": false,
    "include_media_info": false,
    "include_mounted_folders": true,
    "include_non_downloadable_files": true,
    "path": "/Homework/math",
    "recursive": false
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
        ),
        Endpoint::FilesListFolderLongpollPost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "timeout": 30
}"##,
            ),
            Some(
                r##"{
    "changes": true
}"##,
            ),
        ),
        Endpoint::FilesListRevisionsPost => (
            Some(
                r##"{
    "limit": 10,
    "mode": "path",
    "path": "/root/word.docx"
}"##,
            ),
            Some(
                r##"{
    "entries": [
        {
            "client_modified": "2015-05-12T15:50:38Z",
            "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "file_lock_info": {
                "created": "2015-05-12T15:50:38Z",
                "is_lockholder": true,
                "lockholder_name": "Imaginary User"
            },
            "has_explicit_shared_members": false,
            "id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_downloadable": true,
            "name": "Prime_Numbers.txt",
            "path_display": "/Homework/math/Prime_Numbers.txt",
            "path_lower": "/homework/math/prime_numbers.txt",
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
            ],
            "rev": "a1c10ce0dd78",
            "server_modified": "2015-05-12T15:50:38Z",
            "sharing_info": {
                "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "parent_shared_folder_id": "84528192421",
                "read_only": true
            },
            "size": 7212
        }
    ],
    "is_deleted": false
}"##,
            ),
        ),
        Endpoint::FilesLockFileBatchPost => (
            Some(
                r##"{
    "entries": [
        {
            "path": "/John Doe/sample/test.pdf"
        }
    ]
}"##,
            ),
            Some(
                r##"{
    "entries": [
        {
            ".tag": "success",
            "lock": {
                "content": {
                    ".tag": "single_user",
                    "created": "2015-05-12T15:50:38Z",
                    "lock_holder_account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "lock_holder_team_id": "dbtid:1234abcd"
                }
            },
            "metadata": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesMovePost => (
            Some(
                r##"{
    "allow_ownership_transfer": false,
    "allow_shared_folder": false,
    "autorename": false,
    "from_path": "/Homework/math",
    "to_path": "/Homework/algebra"
}"##,
            ),
            Some(
                r##"{
    "metadata": {
        ".tag": "file",
        "client_modified": "2015-05-12T15:50:38Z",
        "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "file_lock_info": {
            "created": "2015-05-12T15:50:38Z",
            "is_lockholder": true,
            "lockholder_name": "Imaginary User"
        },
        "has_explicit_shared_members": false,
        "id": "id:a4ayc_80_OEAAAAAAAAAXw",
        "is_downloadable": true,
        "name": "Prime_Numbers.txt",
        "path_display": "/Homework/math/Prime_Numbers.txt",
        "path_lower": "/homework/math/prime_numbers.txt",
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
        ],
        "rev": "a1c10ce0dd78",
        "server_modified": "2015-05-12T15:50:38Z",
        "sharing_info": {
            "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
            "parent_shared_folder_id": "84528192421",
            "read_only": true
        },
        "size": 7212
    }
}"##,
            ),
        ),
        Endpoint::FilesMoveBatchPost => (
            Some(
                r##"{
    "allow_ownership_transfer": false,
    "autorename": false,
    "entries": [
        {
            "from_path": "/Homework/math",
            "to_path": "/Homework/algebra"
        }
    ]
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "success": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesMoveBatchCheckPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "success": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesPaperCreatePost => (
            Some(
                r##"{
    "import_format": "html",
    "path": "/Paper Docs/New Doc.paper"
}"##,
            ),
            Some(
                r##"{
    "file_id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "paper_revision": 1,
    "result_path": "/Paper Docs/New Doc.paper",
    "url": "https://www.dropbox.com/scl/xxx.paper?dl=0"
}"##,
            ),
        ),
        Endpoint::FilesPaperUpdatePost => (
            Some(
                r##"{
    "doc_update_policy": "update",
    "import_format": "html",
    "paper_revision": 123,
    "path": "/Paper Docs/My Doc.paper"
}"##,
            ),
            Some(
                r##"{
    "paper_revision": 124
}"##,
            ),
        ),
        Endpoint::FilesPermanentlyDeletePost => (
            Some(
                r##"{
    "path": "/Homework/math/Prime_Numbers.txt"
}"##,
            ),
            None,
        ),
        Endpoint::FilesRestorePost => (
            Some(
                r##"S
{
    "path": "/root/word.docx",
    "rev": "a1c10ce0dd78"
}"##,
            ),
            Some(
                r##"{
    "client_modified": "2015-05-12T15:50:38Z",
    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "file_lock_info": {
        "created": "2015-05-12T15:50:38Z",
        "is_lockholder": true,
        "lockholder_name": "Imaginary User"
    },
    "has_explicit_shared_members": false,
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_downloadable": true,
    "name": "Prime_Numbers.txt",
    "path_display": "/Homework/math/Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
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
    ],
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "sharing_info": {
        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
        "parent_shared_folder_id": "84528192421",
        "read_only": true
    },
    "size": 7212
}"##,
            ),
        ),
        Endpoint::FilesSaveUrlPost => (
            Some(
                r##"{
    "path": "/a.txt",
    "url": "http://example.com/a.txt"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "client_modified": "2015-05-12T15:50:38Z",
    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "file_lock_info": {
        "created": "2015-05-12T15:50:38Z",
        "is_lockholder": true,
        "lockholder_name": "Imaginary User"
    },
    "has_explicit_shared_members": false,
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_downloadable": true,
    "name": "Prime_Numbers.txt",
    "path_display": "/Homework/math/Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
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
    ],
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "sharing_info": {
        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
        "parent_shared_folder_id": "84528192421",
        "read_only": true
    },
    "size": 7212
}"##,
            ),
        ),
        Endpoint::FilesSaveUrlCheckJobStatusPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
            Some(
                r##"{
    ".tag": "in_progress"
}"##,
            ),
        ),
        Endpoint::FilesSearchPost => (
            Some(
                r##"{
    "match_field_options": {
        "include_highlights": false
    },
    "options": {
        "file_status": "active",
        "filename_only": false,
        "max_results": 20,
        "path": "/Folder"
    },
    "query": "cat"
}"##,
            ),
            Some(
                r##"{
    "has_more": false,
    "matches": [
        {
            "metadata": {
                ".tag": "metadata",
                "metadata": {
                    ".tag": "file",
                    "client_modified": "2015-05-12T15:50:38Z",
                    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                    "has_explicit_shared_members": false,
                    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                    "is_downloadable": true,
                    "name": "Prime_Numbers.txt",
                    "path_display": "/Homework/math/Prime_Numbers.txt",
                    "path_lower": "/homework/math/prime_numbers.txt",
                    "rev": "a1c10ce0dd78",
                    "server_modified": "2015-05-12T15:50:38Z",
                    "sharing_info": {
                        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                        "parent_shared_folder_id": "84528192421",
                        "read_only": true
                    },
                    "size": 7212
                }
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesSearchContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "has_more": false,
    "matches": [
        {
            "metadata": {
                ".tag": "metadata",
                "metadata": {
                    ".tag": "file",
                    "client_modified": "2015-05-12T15:50:38Z",
                    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                    "has_explicit_shared_members": false,
                    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                    "is_downloadable": true,
                    "name": "Prime_Numbers.txt",
                    "path_display": "/Homework/math/Prime_Numbers.txt",
                    "path_lower": "/homework/math/prime_numbers.txt",
                    "rev": "a1c10ce0dd78",
                    "server_modified": "2015-05-12T15:50:38Z",
                    "sharing_info": {
                        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                        "parent_shared_folder_id": "84528192421",
                        "read_only": true
                    },
                    "size": 7212
                }
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesTagsAddPost => (
            Some(
                r##"{
    "path": "/Prime_Numbers.txt",
    "tag_text": "my_tag"
}"##,
            ),
            None,
        ),
        Endpoint::FilesTagsGetPost => (
            Some(
                r##"{
    "paths": [
        "/Prime_Numbers.txt"
    ]
}"##,
            ),
            Some(
                r##"{
    "paths_to_tags": [
        {
            "path": "/Prime_Numbers.txt",
            "tags": [
                {
                    ".tag": "user_generated_tag",
                    "tag_text": "my_tag"
                }
            ]
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesTagsRemovePost => (
            Some(
                r##"{
    "path": "/Prime_Numbers.txt",
    "tag_text": "my_tag"
}"##,
            ),
            None,
        ),
        Endpoint::FilesUnlockFileBatchPost => (
            Some(
                r##"{
    "entries": [
        {
            "path": "/John Doe/sample/test.pdf"
        }
    ]
}"##,
            ),
            Some(
                r##"{
    "entries": [
        {
            ".tag": "success",
            "lock": {
                "content": {
                    ".tag": "single_user",
                    "created": "2015-05-12T15:50:38Z",
                    "lock_holder_account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "lock_holder_team_id": "dbtid:1234abcd"
                }
            },
            "metadata": {
                ".tag": "file",
                "client_modified": "2015-05-12T15:50:38Z",
                "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "file_lock_info": {
                    "created": "2015-05-12T15:50:38Z",
                    "is_lockholder": true,
                    "lockholder_name": "Imaginary User"
                },
                "has_explicit_shared_members": false,
                "id": "id:a4ayc_80_OEAAAAAAAAAXw",
                "is_downloadable": true,
                "name": "Prime_Numbers.txt",
                "path_display": "/Homework/math/Prime_Numbers.txt",
                "path_lower": "/homework/math/prime_numbers.txt",
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
                ],
                "rev": "a1c10ce0dd78",
                "server_modified": "2015-05-12T15:50:38Z",
                "sharing_info": {
                    "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                    "parent_shared_folder_id": "84528192421",
                    "read_only": true
                },
                "size": 7212
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesUploadPost => (
            Some(
                r##"{
    "autorename": false,
    "mode": "add",
    "mute": false,
    "path": "/Homework/math/Matrices.txt",
    "strict_conflict": false
}"##,
            ),
            Some(
                r##"{
    "client_modified": "2015-05-12T15:50:38Z",
    "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "file_lock_info": {
        "created": "2015-05-12T15:50:38Z",
        "is_lockholder": true,
        "lockholder_name": "Imaginary User"
    },
    "has_explicit_shared_members": false,
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_downloadable": true,
    "name": "Prime_Numbers.txt",
    "path_display": "/Homework/math/Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
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
    ],
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "sharing_info": {
        "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
        "parent_shared_folder_id": "84528192421",
        "read_only": true
    },
    "size": 7212
}"##,
            ),
        ),
        Endpoint::FilesUploadSessionAppendPost => (
            Some(
                r##"{
    "close": false,
    "cursor": {
        "offset": 0,
        "session_id": "1234faaf0678bcde"
    }
}"##,
            ),
            None,
        ),
        Endpoint::FilesUploadSessionAppendBatchPost => (
            Some(
                r##"{
    "entries": [
        {
            "close": false,
            "cursor": {
                "offset": 0,
                "session_id": "1234faaf0678bcde"
            },
            "length": 12345
        }
    ]
}"##,
            ),
            Some(
                r##"{
    "entries": [
        {
            "close": false,
            "cursor": {
                "offset": 0,
                "session_id": "1234faaf0678bcde"
            },
            "length": 12345
        },
        {
            "close": false,
            "cursor": {
                "offset": 1073741824,
                "session_id": "8dd9d57374911153"
            },
            "length": 67890
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesUploadSessionFinishPost => (
            Some(
                r##"{
    "commit": {
        "autorename": true,
        "mode": "add",
        "mute": false,
        "path": "/Homework/math/Matrices.txt",
        "strict_conflict": false
    },
    "cursor": {
        "offset": 0,
        "session_id": "1234faaf0678bcde"
    }
}"##,
            ),
            Some(
                r##"{
    "commit": {
        "autorename": true,
        "mode": "add",
        "mute": false,
        "path": "/Homework/math/Vectors.txt",
        "strict_conflict": false
    },
    "cursor": {
        "offset": 1073741824,
        "session_id": "8dd9d57374911153"
    }
}
"##,
            ),
        ),
        Endpoint::FilesUploadSessionFinishBatchPost => (
            Some(
                r##"{
    "entries": [
        {
            "commit": {
                "autorename": true,
                "mode": "add",
                "mute": false,
                "path": "/Homework/math/Matrices.txt",
                "strict_conflict": false
            },
            "cursor": {
                "offset": 0,
                "session_id": "1234faaf0678bcde"
            }
        }
    ]
}"##,
            ),
            Some(
                r##"{
    "entries": [
        {
            "commit": {
                "autorename": true,
                "mode": "add",
                "mute": false,
                "path": "/Homework/math/Matrices.txt",
                "strict_conflict": false
            },
            "cursor": {
                "offset": 0,
                "session_id": "1234faaf0678bcde"
            }
        },
        {
            "commit": {
                "autorename": true,
                "mode": "add",
                "mute": false,
                "path": "/Homework/math/Vectors.txt",
                "strict_conflict": false
            },
            "cursor": {
                "offset": 1073741824,
                "session_id": "8dd9d57374911153"
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesUploadSessionFinishBatchCheckPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "entries": [
        {
            ".tag": "success",
            "client_modified": "2015-05-12T15:50:38Z",
            "content_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "file_lock_info": {
                "created": "2015-05-12T15:50:38Z",
                "is_lockholder": true,
                "lockholder_name": "Imaginary User"
            },
            "has_explicit_shared_members": false,
            "id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_downloadable": true,
            "name": "Prime_Numbers.txt",
            "path_display": "/Homework/math/Prime_Numbers.txt",
            "path_lower": "/homework/math/prime_numbers.txt",
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
            ],
            "rev": "a1c10ce0dd78",
            "server_modified": "2015-05-12T15:50:38Z",
            "sharing_info": {
                "modified_by": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "parent_shared_folder_id": "84528192421",
                "read_only": true
            },
            "size": 7212
        }
    ]
}"##,
            ),
        ),
        Endpoint::FilesUploadSessionStartPost => (
            Some(
                r##"{
    "close": false
}"##,
            ),
            Some(
                r##"{
    "session_id": "1234faaf0678bcde"
}"##,
            ),
        ),
        Endpoint::FilesUploadSessionStartBatchPost => (
            Some(
                r##"{
    "num_sessions": 1
}"##,
            ),
            Some(
                r##"{
    "session_ids": [
        "1234faaf0678bcde"
    ]
}"##,
            ),
        ),
        Endpoint::OpenidUserInfoPost => (
            None,
            Some(
                r##"{
    "family_name": "Doe",
    "given_name": "John",
    "email": "john.doe@example.com",
    "email_verified": true,
    "iss": "Dropbox",
    "sub": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc"
}"##,
            ),
        ),
        Endpoint::AuthTokenRevokePost => (None, None),
        Endpoint::CheckUserPost => (
            Some(
                r##"S
{
    "query": "foo"
}"##,
            ),
            Some(
                r##"{
    "result": "foo"
}
"##,
            ),
        ),
        Endpoint::SharingAddFileMemberPost => (
            Some(
                r##"{
    "access_level": "viewer",
    "add_message_as_comment": false,
    "custom_message": "This is a custom message about ACME.doc",
    "file": "id:3kmLmQFnf1AAAAAAAAAAAw",
    "members": [
        {
            ".tag": "email",
            "email": "justin@example.com"
        }
    ],
    "quiet": false
}"##,
            ),
            Some(
                r##"{
    "results": [
        {
            "member": {
                "dropbox_id": "1234567890abcdef"
            },
            "result": {
                "success": {
                    "access_level": {
                        "level": "viewer"
                    }
                }
            }
        },
        {
            "member": {
                "email": "user@example.com"
            },
            "result": {
                "member_error": {
                    "error_type": "no_permission"
                }
            }
        },
        {
            "member": {
                "dropbox_id": "abcdef1234567890"
            },
            "result": {
                "success": {
                    "access_level": {
                        "level": "editor"
                    }
                },
                "sckey_sha1": "1234567890abcdef1234567890abcdef12345678",
                "invitation_signature": [
                    "signature1",
                    "signature2"
                ]
            }
        }
    ]
}
"##,
            ),
        ),
        Endpoint::SharingAddFolderMemberPost => (
            Some(
                r##"{
    "custom_message": "Documentation for launch day",
    "members": [
        {
            "access_level": "editor",
            "member": {
                ".tag": "email",
                "email": "justin@example.com"
            }
        },
        {
            "access_level": "viewer",
            "member": {
                ".tag": "dropbox_id",
                "dropbox_id": "dbid:AAEufNrMPSPe0dMQijRP0N_aZtBJRm26W4Q"
            }
        }
    ],
    "quiet": false,
    "shared_folder_id": "84528192421"
}"##,
            ),
            None,
        ),
        Endpoint::SharingCheckJobStatusPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
            Some(
                r##"{
    ".tag": "in_progress"
}"##,
            ),
        ),
        Endpoint::SharingCheckRemoveMemberJobStatusPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
            Some(
                r##"
{
    ".tag": "complete"
}"##,
            ),
        ),
        Endpoint::SharingCheckShareJobStatusPost => (
            Some(
                r##"{
    "async_job_id": "34g93hh34h04y384084"
}
"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "access_inheritance": {
        ".tag": "inherit"
    },
    "access_type": {
        ".tag": "owner"
    },
    "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_inside_team_folder": false,
    "is_team_folder": false,
    "link_metadata": {
        "audience_options": [
            {
                ".tag": "public"
            },
            {
                ".tag": "team"
            },
            {
                ".tag": "members"
            }
        ],
        "current_audience": {
            ".tag": "public"
        },
        "link_permissions": [
            {
                "action": {
                    ".tag": "change_audience"
                },
                "allow": true
            }
        ],
        "password_protected": false,
        "url": ""
    },
    "name": "dir",
    "path_lower": "/dir",
    "permissions": [],
    "policy": {
        "acl_update_policy": {
            ".tag": "owner"
        },
        "member_policy": {
            ".tag": "anyone"
        },
        "resolved_member_policy": {
            ".tag": "team"
        },
        "shared_link_policy": {
            ".tag": "anyone"
        }
    },
    "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
    "shared_folder_id": "84528192421",
    "time_invited": "2016-01-20T00:00:00Z"
}"##,
            ),
        ),
        Endpoint::SharingCreateSharedLinkWithSettingsPost => (
            Some(
                r##"    "path": "/Prime_Numbers.txt",
    "settings": {
        "access": "viewer",
        "allow_download": true,
        "audience": "public",
        "requested_visibility": "public"
    }
}"##,
            ),
            Some(
                r##"{
    ".tag": "file",
    "client_modified": "2015-05-12T15:50:38Z",
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "link_permissions": {
        "allow_comments": true,
        "allow_download": true,
        "audience_options": [
            {
                "allowed": true,
                "audience": {
                    ".tag": "public"
                }
            },
            {
                "allowed": false,
                "audience": {
                    ".tag": "team"
                }
            },
            {
                "allowed": true,
                "audience": {
                    ".tag": "no_one"
                }
            }
        ],
        "can_allow_download": true,
        "can_disallow_download": false,
        "can_remove_expiry": false,
        "can_remove_password": true,
        "can_revoke": false,
        "can_set_expiry": false,
        "can_set_password": true,
        "can_use_extended_sharing_controls": false,
        "require_password": false,
        "resolved_visibility": {
            ".tag": "public"
        },
        "revoke_failure_reason": {
            ".tag": "owner_only"
        },
        "team_restricts_comments": true,
        "visibility_policies": [
            {
                "allowed": true,
                "policy": {
                    ".tag": "public"
                },
                "resolved_policy": {
                    ".tag": "public"
                }
            },
            {
                "allowed": true,
                "policy": {
                    ".tag": "password"
                },
                "resolved_policy": {
                    ".tag": "password"
                }
            }
        ]
    },
    "name": "Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "size": 7212,
    "team_member_info": {
        "display_name": "Roger Rabbit",
        "member_id": "dbmid:abcd1234",
        "team_info": {
            "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
            "name": "Acme, Inc."
        }
    },
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
        ),
        Endpoint::SharingGetFileMetadataPost => (
            Some(
                r##"{
    "actions": [],
    "file": "id:3kmLmQFnf1AAAAAAAAAAAw"
}"##,
            ),
            Some(
                r##"{
    "access_type": {
        ".tag": "viewer"
    },
    "id": "id:3kmLmQFnf1AAAAAAAAAAAw",
    "name": "file.txt",
    "owner_display_names": [
        "Jane Doe"
    ],
    "owner_team": {
        "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
        "name": "Acme, Inc."
    },
    "path_display": "/dir/file.txt",
    "path_lower": "/dir/file.txt",
    "permissions": [],
    "policy": {
        "acl_update_policy": {
            ".tag": "owner"
        },
        "member_policy": {
            ".tag": "anyone"
        },
        "resolved_member_policy": {
            ".tag": "team"
        },
        "shared_link_policy": {
            ".tag": "anyone"
        }
    },
    "preview_url": "https://www.dropbox.com/scl/fi/fir9vjelf",
    "time_invited": "2016-01-20T00:00:00Z"
}"##,
            ),
        ),
        Endpoint::SharingGetFileMetadataBatchPost => (
            Some(
                r##"{
    "actions": [],
    "files": [
        "id:3kmLmQFnf1AAAAAAAAAAAw",
        "id:VvTaJu2VZzAAAAAAAAAADQ"
    ]
}"##,
            ),
            Some(
                r##"{
  "files": [
    {
      "file": "id:1234567890abcdef",
      "result": {
        "metadata": {
          "id": "id:1234567890abcdef",
          "name": "example_file.txt",
          "policy": {
            "acl_update_policy": "editors",
            "shared_link_policy": "team_only",
            "member_policy": "team"
          },
          "preview_url": "https://www.dropbox.com/preview/example_file.txt",
          "access_type": "viewer",
          "owner_display_names": ["John Doe", "Jane Smith"],
          "owner_team": {
            "id": "team123",
            "name": "Example Team"
          },
          "parent_shared_folder_id": "folder1234567890abcdef",
          "path_display": "/Example Folder/example_file.txt",
          "path_lower": "/example folder/example_file.txt",
          "permissions": [
            {
              "action": "edit",
              "allow": false,
              "reason": "insufficient_permissions"
            },
            {
              "action": "comment",
              "allow": true,
              "reason": "allowed"
            }
          ],
          "time_invited": "2023-08-01T12:00:00Z"
        }
      }
    },
    {
      "file": "id:abcdef1234567890",
      "result": {
        "access_error": {
          "no_permission": {}
        }
      }
    },
    {
      "file": "/path/to/file2.txt",
      "result": {
        "metadata": {
          "id": "id:abcdef1234567890",
          "name": "file2.txt",
          "policy": {
            "acl_update_policy": "owners",
            "shared_link_policy": "anyone",
            "member_policy": "team"
          },
          "preview_url": "https://www.dropbox.com/preview/file2.txt",
          "access_type": "editor",
          "owner_display_names": ["Alice Johnson"],
          "owner_team": {
            "id": "team456",
            "name": "Another Team"
          },
          "parent_shared_folder_id": "folderabcdef1234567890",
          "path_display": "/Another Folder/file2.txt",
          "path_lower": "/another folder/file2.txt",
          "permissions": [
            {
              "action": "edit",
              "allow": true,
              "reason": "allowed"
            },
            {
              "action": "share",
              "allow": true,
              "reason": "allowed"
            }
          ],
          "time_invited": "2023-07-15T08:30:00Z"
        }
      }
    },
    {
      "file": "nspath:12345:/path/to/another/file3.txt",
      "result": {
        "access_error": {
          "invalid_file": {}
        }
      }
    },
    {
      "file": "id:1122334455667788",
      "result": {
        "access_error": {
          "is_folder": {}
        }
      }
    }
  ]
}
"##,
            ),
        ),
        Endpoint::SharingGetFolderMetadataPost => (
            Some(
                r##"{
    "actions": [],
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
    "access_inheritance": {
        ".tag": "inherit"
    },
    "access_type": {
        ".tag": "owner"
    },
    "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_inside_team_folder": false,
    "is_team_folder": false,
    "link_metadata": {
        "audience_options": [
            {
                ".tag": "public"
            },
            {
                ".tag": "team"
            },
            {
                ".tag": "members"
            }
        ],
        "current_audience": {
            ".tag": "public"
        },
        "link_permissions": [
            {
                "action": {
                    ".tag": "change_audience"
                },
                "allow": true
            }
        ],
        "password_protected": false,
        "url": ""
    },
    "name": "dir",
    "path_lower": "/dir",
    "permissions": [],
    "policy": {
        "acl_update_policy": {
            ".tag": "owner"
        },
        "member_policy": {
            ".tag": "anyone"
        },
        "resolved_member_policy": {
            ".tag": "team"
        },
        "shared_link_policy": {
            ".tag": "anyone"
        }
    },
    "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
    "shared_folder_id": "84528192421",
    "time_invited": "2016-01-20T00:00:00Z"
}"##,
            ),
        ),
        Endpoint::SharingGetSharedLinkFilePost => (
            Some(
                r##"{
    "path": "/Prime_Numbers.txt",
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
            Some(
                r##"{
    ".tag": "file",
    "client_modified": "2015-05-12T15:50:38Z",
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "link_permissions": {
        "allow_comments": true,
        "allow_download": true,
        "audience_options": [
            {
                "allowed": true,
                "audience": {
                    ".tag": "public"
                }
            },
            {
                "allowed": false,
                "audience": {
                    ".tag": "team"
                }
            },
            {
                "allowed": true,
                "audience": {
                    ".tag": "no_one"
                }
            }
        ],
        "can_allow_download": true,
        "can_disallow_download": false,
        "can_remove_expiry": false,
        "can_remove_password": true,
        "can_revoke": false,
        "can_set_expiry": false,
        "can_set_password": true,
        "can_use_extended_sharing_controls": false,
        "require_password": false,
        "resolved_visibility": {
            ".tag": "public"
        },
        "revoke_failure_reason": {
            ".tag": "owner_only"
        },
        "team_restricts_comments": true,
        "visibility_policies": [
            {
                "allowed": true,
                "policy": {
                    ".tag": "public"
                },
                "resolved_policy": {
                    ".tag": "public"
                }
            },
            {
                "allowed": true,
                "policy": {
                    ".tag": "password"
                },
                "resolved_policy": {
                    ".tag": "password"
                }
            }
        ]
    },
    "name": "Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "size": 7212,
    "team_member_info": {
        "display_name": "Roger Rabbit",
        "member_id": "dbmid:abcd1234",
        "team_info": {
            "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
            "name": "Acme, Inc."
        }
    },
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
        ),
        Endpoint::SharingGetSharedLinkMetadataPost => (
            Some(
                r##"{
    "path": "/Prime_Numbers.txt",
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
            Some(
                r##"{
    ".tag": "file",
    "client_modified": "2015-05-12T15:50:38Z",
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "link_permissions": {
        "allow_comments": true,
        "allow_download": true,
        "audience_options": [
            {
                "allowed": true,
                "audience": {
                    ".tag": "public"
                }
            },
            {
                "allowed": false,
                "audience": {
                    ".tag": "team"
                }
            },
            {
                "allowed": true,
                "audience": {
                    ".tag": "no_one"
                }
            }
        ],
        "can_allow_download": true,
        "can_disallow_download": false,
        "can_remove_expiry": false,
        "can_remove_password": true,
        "can_revoke": false,
        "can_set_expiry": false,
        "can_set_password": true,
        "can_use_extended_sharing_controls": false,
        "require_password": false,
        "resolved_visibility": {
            ".tag": "public"
        },
        "revoke_failure_reason": {
            ".tag": "owner_only"
        },
        "team_restricts_comments": true,
        "visibility_policies": [
            {
                "allowed": true,
                "policy": {
                    ".tag": "public"
                },
                "resolved_policy": {
                    ".tag": "public"
                }
            },
            {
                "allowed": true,
                "policy": {
                    ".tag": "password"
                },
                "resolved_policy": {
                    ".tag": "password"
                }
            }
        ]
    },
    "name": "Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "size": 7212,
    "team_member_info": {
        "display_name": "Roger Rabbit",
        "member_id": "dbmid:abcd1234",
        "team_info": {
            "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
            "name": "Acme, Inc."
        }
    },
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
        ),
        Endpoint::SharingListFileMembersPost => (
            Some(
                r##"{
    "file": "id:3kmLmQFnf1AAAAAAAAAAAw",
    "include_inherited": true,
    "limit": 100
}"##,
            ),
            Some(
                r##"{
    "groups": [
        {
            "access_type": {
                ".tag": "editor"
            },
            "group": {
                "group_id": "g:e2db7665347abcd600000000001a2b3c",
                "group_management_type": {
                    ".tag": "user_managed"
                },
                "group_name": "Test group",
                "group_type": {
                    ".tag": "user_managed"
                },
                "is_member": false,
                "is_owner": false,
                "member_count": 10,
                "same_team": true
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "invitees": [
        {
            "access_type": {
                ".tag": "viewer"
            },
            "invitee": {
                ".tag": "email",
                "email": "jessica@example.com"
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "users": [
        {
            "access_type": {
                ".tag": "owner"
            },
            "is_inherited": false,
            "permissions": [],
            "platform_type": {
                ".tag": "unknown"
            },
            "time_last_seen": "2016-01-20T00:00:00Z",
            "user": {
                "account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "display_name": "Robert Smith",
                "email": "bob@example.com",
                "same_team": true,
                "team_member_id": "dbmid:abcd1234"
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListFileMembersBatchPost => (
            Some(
                r##"    "files": [
        "id:3kmLmQFnf1AAAAAAAAAAAw",
        "id:VvTaJu2VZzAAAAAAAAAADQ"
    ],
    "limit": 10
}"##,
            ),
            Some(
                r##"{
  "files": [
    {
      "file": "id:1234567890abcdef",
      "result": {
        "result": {
          "members": {
            "users": [
              {
                "account_id": "1234567890abcdefghijklmnopqrstuvwxyzabcd",
                "email": "john.doe@example.com",
                "access_level": "viewer"
              },
              {
                "account_id": "abcdefghijklmnopqrstuvwxyzabcdef1234567890",
                "email": "jane.smith@example.com",
                "access_level": "editor"
              }
            ]
          },
          "member_count": 2
        }
      }
    },
    {
      "file": "/path/to/file.txt",
      "result": {
        "access_error": {
          "no_permission": {}
        }
      }
    },
    {
      "file": "id:abcdef1234567890",
      "result": {
        "access_error": {
          "invalid_file": {}
        }
      }
    },
    {
      "file": "nspath:12345:/path/to/another/file.txt",
      "result": {
        "result": {
          "members": {
            "users": [
              {
                "account_id": "abcdef1234567890abcdefghijklmnopqrstuvwxyz",
                "email": "alice.johnson@example.com",
                "access_level": "viewer"
              }
            ]
          },
          "member_count": 1
        }
      }
    },
    {
      "file": "id:1122334455667788",
      "result": {
        "access_error": {
          "is_folder": {}
        }
      }
    }
  ]
}
"##,
            ),
        ),
        Endpoint::SharingListFileMembersContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "groups": [
        {
            "access_type": {
                ".tag": "editor"
            },
            "group": {
                "group_id": "g:e2db7665347abcd600000000001a2b3c",
                "group_management_type": {
                    ".tag": "user_managed"
                },
                "group_name": "Test group",
                "group_type": {
                    ".tag": "user_managed"
                },
                "is_member": false,
                "is_owner": false,
                "member_count": 10,
                "same_team": true
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "invitees": [
        {
            "access_type": {
                ".tag": "viewer"
            },
            "invitee": {
                ".tag": "email",
                "email": "jessica@example.com"
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "users": [
        {
            "access_type": {
                ".tag": "owner"
            },
            "is_inherited": false,
            "permissions": [],
            "platform_type": {
                ".tag": "unknown"
            },
            "time_last_seen": "2016-01-20T00:00:00Z",
            "user": {
                "account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "display_name": "Robert Smith",
                "email": "bob@example.com",
                "same_team": true,
                "team_member_id": "dbmid:abcd1234"
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListFolderMembersPost => (
            Some(
                r##"{
    "actions": [],
    "limit": 10,
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "groups": [
        {
            "access_type": {
                ".tag": "editor"
            },
            "group": {
                "group_id": "g:e2db7665347abcd600000000001a2b3c",
                "group_management_type": {
                    ".tag": "user_managed"
                },
                "group_name": "Test group",
                "group_type": {
                    ".tag": "user_managed"
                },
                "is_member": false,
                "is_owner": false,
                "member_count": 10,
                "same_team": true
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "invitees": [
        {
            "access_type": {
                ".tag": "viewer"
            },
            "invitee": {
                ".tag": "email",
                "email": "jessica@example.com"
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "users": [
        {
            "access_type": {
                ".tag": "owner"
            },
            "is_inherited": false,
            "permissions": [],
            "user": {
                "account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "display_name": "Robert Smith",
                "email": "bob@example.com",
                "same_team": true,
                "team_member_id": "dbmid:abcd1234"
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListFolderMembersContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "groups": [
        {
            "access_type": {
                ".tag": "editor"
            },
            "group": {
                "group_id": "g:e2db7665347abcd600000000001a2b3c",
                "group_management_type": {
                    ".tag": "user_managed"
                },
                "group_name": "Test group",
                "group_type": {
                    ".tag": "user_managed"
                },
                "is_member": false,
                "is_owner": false,
                "member_count": 10,
                "same_team": true
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "invitees": [
        {
            "access_type": {
                ".tag": "viewer"
            },
            "invitee": {
                ".tag": "email",
                "email": "jessica@example.com"
            },
            "is_inherited": false,
            "permissions": []
        }
    ],
    "users": [
        {
            "access_type": {
                ".tag": "owner"
            },
            "is_inherited": false,
            "permissions": [],
            "user": {
                "account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
                "display_name": "Robert Smith",
                "email": "bob@example.com",
                "same_team": true,
                "team_member_id": "dbmid:abcd1234"
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListFoldersPost => (
            Some(
                r##"{
    "actions": [],
    "limit": 100
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "entries": [
        {
            "access_inheritance": {
                ".tag": "inherit"
            },
            "access_type": {
                ".tag": "owner"
            },
            "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_inside_team_folder": false,
            "is_team_folder": false,
            "link_metadata": {
                "audience_options": [
                    {
                        ".tag": "public"
                    },
                    {
                        ".tag": "team"
                    },
                    {
                        ".tag": "members"
                    }
                ],
                "current_audience": {
                    ".tag": "public"
                },
                "link_permissions": [
                    {
                        "action": {
                            ".tag": "change_audience"
                        },
                        "allow": true
                    }
                ],
                "password_protected": false,
                "url": ""
            },
            "name": "dir",
            "path_lower": "/dir",
            "permissions": [],
            "policy": {
                "acl_update_policy": {
                    ".tag": "owner"
                },
                "member_policy": {
                    ".tag": "anyone"
                },
                "resolved_member_policy": {
                    ".tag": "team"
                },
                "shared_link_policy": {
                    ".tag": "anyone"
                }
            },
            "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
            "shared_folder_id": "84528192421",
            "time_invited": "2016-01-20T00:00:00Z"
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListFoldersContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "entries": [
        {
            "access_inheritance": {
                ".tag": "inherit"
            },
            "access_type": {
                ".tag": "owner"
            },
            "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_inside_team_folder": false,
            "is_team_folder": false,
            "link_metadata": {
                "audience_options": [
                    {
                        ".tag": "public"
                    },
                    {
                        ".tag": "team"
                    },
                    {
                        ".tag": "members"
                    }
                ],
                "current_audience": {
                    ".tag": "public"
                },
                "link_permissions": [
                    {
                        "action": {
                            ".tag": "change_audience"
                        },
                        "allow": true
                    }
                ],
                "password_protected": false,
                "url": ""
            },
            "name": "dir",
            "path_lower": "/dir",
            "permissions": [],
            "policy": {
                "acl_update_policy": {
                    ".tag": "owner"
                },
                "member_policy": {
                    ".tag": "anyone"
                },
                "resolved_member_policy": {
                    ".tag": "team"
                },
                "shared_link_policy": {
                    ".tag": "anyone"
                }
            },
            "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
            "shared_folder_id": "84528192421",
            "time_invited": "2016-01-20T00:00:00Z"
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListMountableFoldersPost => (
            Some(
                r##"{
    "actions": [],
    "limit": 100
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "entries": [
        {
            "access_inheritance": {
                ".tag": "inherit"
            },
            "access_type": {
                ".tag": "owner"
            },
            "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_inside_team_folder": false,
            "is_team_folder": false,
            "link_metadata": {
                "audience_options": [
                    {
                        ".tag": "public"
                    },
                    {
                        ".tag": "team"
                    },
                    {
                        ".tag": "members"
                    }
                ],
                "current_audience": {
                    ".tag": "public"
                },
                "link_permissions": [
                    {
                        "action": {
                            ".tag": "change_audience"
                        },
                        "allow": true
                    }
                ],
                "password_protected": false,
                "url": ""
            },
            "name": "dir",
            "path_lower": "/dir",
            "permissions": [],
            "policy": {
                "acl_update_policy": {
                    ".tag": "owner"
                },
                "member_policy": {
                    ".tag": "anyone"
                },
                "resolved_member_policy": {
                    ".tag": "team"
                },
                "shared_link_policy": {
                    ".tag": "anyone"
                }
            },
            "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
            "shared_folder_id": "84528192421",
            "time_invited": "2016-01-20T00:00:00Z"
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListMountableFoldersContinuePost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu",
    "entries": [
        {
            "access_inheritance": {
                ".tag": "inherit"
            },
            "access_type": {
                ".tag": "owner"
            },
            "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
            "is_inside_team_folder": false,
            "is_team_folder": false,
            "link_metadata": {
                "audience_options": [
                    {
                        ".tag": "public"
                    },
                    {
                        ".tag": "team"
                    },
                    {
                        ".tag": "members"
                    }
                ],
                "current_audience": {
                    ".tag": "public"
                },
                "link_permissions": [
                    {
                        "action": {
                            ".tag": "change_audience"
                        },
                        "allow": true
                    }
                ],
                "password_protected": false,
                "url": ""
            },
            "name": "dir",
            "path_lower": "/dir",
            "permissions": [],
            "policy": {
                "acl_update_policy": {
                    ".tag": "owner"
                },
                "member_policy": {
                    ".tag": "anyone"
                },
                "resolved_member_policy": {
                    ".tag": "team"
                },
                "shared_link_policy": {
                    ".tag": "anyone"
                }
            },
            "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
            "shared_folder_id": "84528192421",
            "time_invited": "2016-01-20T00:00:00Z"
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListReceivedFilesPost => (
            Some(
                r##"{
    "actions": [],
    "limit": 100
}"##,
            ),
            Some(
                r##"{
    "cursor": "AzJJbGlzdF90eXBdofe9c3RPbGlzdGFyZ3NfYnlfZ2lkMRhcbric7Rdog9cmV2aXNpb24H3Qf6o1fkHxQ",
    "entries": [
        {
            "access_type": {
                ".tag": "viewer"
            },
            "id": "id:3kmLmQFnf1AAAAAAAAAAAw",
            "name": "file.txt",
            "owner_display_names": [
                "Jane Doe"
            ],
            "owner_team": {
                "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
                "name": "Acme, Inc."
            },
            "path_display": "/dir/file.txt",
            "path_lower": "/dir/file.txt",
            "permissions": [],
            "policy": {
                "acl_update_policy": {
                    ".tag": "owner"
                },
                "member_policy": {
                    ".tag": "anyone"
                },
                "resolved_member_policy": {
                    ".tag": "team"
                },
                "shared_link_policy": {
                    ".tag": "anyone"
                }
            },
            "preview_url": "https://www.dropbox.com/scl/fi/fir9vjelf",
            "time_invited": "2016-01-20T00:00:00Z"
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListReceivedFilesContinuePost => (
            Some(
                r##"{
    "cursor": "AzJJbGlzdF90eXBdofe9c3RPbGlzdGFyZ3NfYnlfZ2lkMRhcbric7Rdog9emfGRlc2MCRWxpbWl0BGRId"
}"##,
            ),
            Some(
                r##"{
    "cursor": "AzJJbGlzdF90eXBdofe9c3RPbGlzdGFyZ3NfYnlfZ2lkMRhcbric7Rdog9cmV2aXNpb24H3Qf6o1fkHxQ",
    "entries": [
        {
            "access_type": {
                ".tag": "viewer"
            },
            "id": "id:3kmLmQFnf1AAAAAAAAAAAw",
            "name": "file.txt",
            "owner_display_names": [
                "Jane Doe"
            ],
            "owner_team": {
                "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
                "name": "Acme, Inc."
            },
            "path_display": "/dir/file.txt",
            "path_lower": "/dir/file.txt",
            "permissions": [],
            "policy": {
                "acl_update_policy": {
                    ".tag": "owner"
                },
                "member_policy": {
                    ".tag": "anyone"
                },
                "resolved_member_policy": {
                    ".tag": "team"
                },
                "shared_link_policy": {
                    ".tag": "anyone"
                }
            },
            "preview_url": "https://www.dropbox.com/scl/fi/fir9vjelf",
            "time_invited": "2016-01-20T00:00:00Z"
        }
    ]
}"##,
            ),
        ),
        Endpoint::SharingListSharedLinksPost => (
            Some(
                r##"{
    "cursor": "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu"
}"##,
            ),
            Some(
                r##"{
    "direct_only": true,
    "path": "id:a4ayc_80_OEAAAAAAAAAYa"
}"##,
            ),
        ),
        Endpoint::SharingModifySharedLinksSettingsPost => (
            Some(
                r##"{
    "remove_expiration": false,
    "settings": {
        "access": "viewer",
        "allow_download": true,
        "audience": "public",
        "requested_visibility": "public"
    },
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
            Some(
                r##"{
    ".tag": "file",
    "client_modified": "2015-05-12T15:50:38Z",
    "id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "link_permissions": {
        "allow_comments": true,
        "allow_download": true,
        "audience_options": [
            {
                "allowed": true,
                "audience": {
                    ".tag": "public"
                }
            },
            {
                "allowed": false,
                "audience": {
                    ".tag": "team"
                }
            },
            {
                "allowed": true,
                "audience": {
                    ".tag": "no_one"
                }
            }
        ],
        "can_allow_download": true,
        "can_disallow_download": false,
        "can_remove_expiry": false,
        "can_remove_password": true,
        "can_revoke": false,
        "can_set_expiry": false,
        "can_set_password": true,
        "can_use_extended_sharing_controls": false,
        "require_password": false,
        "resolved_visibility": {
            ".tag": "public"
        },
        "revoke_failure_reason": {
            ".tag": "owner_only"
        },
        "team_restricts_comments": true,
        "visibility_policies": [
            {
                "allowed": true,
                "policy": {
                    ".tag": "public"
                },
                "resolved_policy": {
                    ".tag": "public"
                }
            },
            {
                "allowed": true,
                "policy": {
                    ".tag": "password"
                },
                "resolved_policy": {
                    ".tag": "password"
                }
            }
        ]
    },
    "name": "Prime_Numbers.txt",
    "path_lower": "/homework/math/prime_numbers.txt",
    "rev": "a1c10ce0dd78",
    "server_modified": "2015-05-12T15:50:38Z",
    "size": 7212,
    "team_member_info": {
        "display_name": "Roger Rabbit",
        "member_id": "dbmid:abcd1234",
        "team_info": {
            "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
            "name": "Acme, Inc."
        }
    },
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
        ),
        Endpoint::SharingMountFolderPost => (
            Some(
                r##"{
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
    "access_inheritance": {
        ".tag": "inherit"
    },
    "access_type": {
        ".tag": "owner"
    },
    "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_inside_team_folder": false,
    "is_team_folder": false,
    "link_metadata": {
        "audience_options": [
            {
                ".tag": "public"
            },
            {
                ".tag": "team"
            },
            {
                ".tag": "members"
            }
        ],
        "current_audience": {
            ".tag": "public"
        },
        "link_permissions": [
            {
                "action": {
                    ".tag": "change_audience"
                },
                "allow": true
            }
        ],
        "password_protected": false,
        "url": ""
    },
    "name": "dir",
    "path_lower": "/dir",
    "permissions": [],
    "policy": {
        "acl_update_policy": {
            ".tag": "owner"
        },
        "member_policy": {
            ".tag": "anyone"
        },
        "resolved_member_policy": {
            ".tag": "team"
        },
        "shared_link_policy": {
            ".tag": "anyone"
        }
    },
    "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
    "shared_folder_id": "84528192421",
    "time_invited": "2016-01-20T00:00:00Z"
}"##,
            ),
        ),
        Endpoint::SharingRelinquishFileMembershipPost => (
            Some(
                r##"{
    "file": "id:3kmLmQFnf1AAAAAAAAAAAw"
}"##,
            ),
            None,
        ),
        Endpoint::SharingRelinquishFolderMembershipPost => (
            Some(
                r##"{
    "leave_a_copy": false,
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete"
}"##,
            ),
        ),
        Endpoint::SharingRemoveFileMember2Post => (
            Some(
                r##"{
    "file": "id:3kmLmQFnf1AAAAAAAAAAAw",
    "member": {
        ".tag": "email",
        "email": "justin@example.com"
    }
}"##,
            ),
            Some(
                r##"{
  "success": {
    "access_level": "viewer",
    "warning": "The user still has access through a shared parent folder.",
    "access_details": [
      {
        "parent_folder_id": "parent1234567890abcdef",
        "parent_folder_name": "Team Shared Folder",
        "access_level": "viewer"
      },
      {
        "parent_folder_id": "parentabcdef1234567890",
        "parent_folder_name": "Project A Folder",
        "access_level": "editor"
      }
    ]
  }
}
"##,
            ),
        ),
        Endpoint::SharingRemoveFolderMemberPost => (
            Some(
                r##"{
    "leave_a_copy": false,
    "member": {
        ".tag": "email",
        "email": "justin@example.com"
    },
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
    ".tag": "async_job_id",
    "async_job_id": "34g93hh34h04y384084"
}"##,
            ),
        ),
        Endpoint::SharingRevokeSharedLinkPost => (
            Some(
                r##"{
    "url": "https://www.dropbox.com/s/2sn712vy1ovegw8/Prime_Numbers.txt?dl=0"
}"##,
            ),
            None,
        ),
        Endpoint::SharingSetAccessInheritancePost => (
            Some(
                r##"{
    "access_inheritance": "inherit",
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "access_inheritance": {
        ".tag": "inherit"
    },
    "access_type": {
        ".tag": "owner"
    },
    "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_inside_team_folder": false,
    "is_team_folder": false,
    "link_metadata": {
        "audience_options": [
            {
                ".tag": "public"
            },
            {
                ".tag": "team"
            },
            {
                ".tag": "members"
            }
        ],
        "current_audience": {
            ".tag": "public"
        },
        "link_permissions": [
            {
                "action": {
                    ".tag": "change_audience"
                },
                "allow": true
            }
        ],
        "password_protected": false,
        "url": ""
    },
    "name": "dir",
    "path_lower": "/dir",
    "permissions": [],
    "policy": {
        "acl_update_policy": {
            ".tag": "owner"
        },
        "member_policy": {
            ".tag": "anyone"
        },
        "resolved_member_policy": {
            ".tag": "team"
        },
        "shared_link_policy": {
            ".tag": "anyone"
        }
    },
    "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
    "shared_folder_id": "84528192421",
    "time_invited": "2016-01-20T00:00:00Z"
}"##,
            ),
        ),
        Endpoint::SharingShareFolderPost => (
            Some(
                r##"{
    "access_inheritance": "inherit",
    "acl_update_policy": "editors",
    "force_async": false,
    "member_policy": "team",
    "path": "/example/workspace",
    "shared_link_policy": "members"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete",
    "access_inheritance": {
        ".tag": "inherit"
    },
    "access_type": {
        ".tag": "owner"
    },
    "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_inside_team_folder": false,
    "is_team_folder": false,
    "link_metadata": {
        "audience_options": [
            {
                ".tag": "public"
            },
            {
                ".tag": "team"
            },
            {
                ".tag": "members"
            }
        ],
        "current_audience": {
            ".tag": "public"
        },
        "link_permissions": [
            {
                "action": {
                    ".tag": "change_audience"
                },
                "allow": true
            }
        ],
        "password_protected": false,
        "url": ""
    },
    "name": "dir",
    "path_lower": "/dir",
    "permissions": [],
    "policy": {
        "acl_update_policy": {
            ".tag": "owner"
        },
        "member_policy": {
            ".tag": "anyone"
        },
        "resolved_member_policy": {
            ".tag": "team"
        },
        "shared_link_policy": {
            ".tag": "anyone"
        }
    },
    "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
    "shared_folder_id": "84528192421",
    "time_invited": "2016-01-20T00:00:00Z"
}"##,
            ),
        ),
        Endpoint::SharingTransferFolderPost => (
            Some(
                r##"{
    "shared_folder_id": "84528192421",
    "to_dropbox_id": "dbid:AAEufNrMPSPe0dMQijRP0N_aZtBJRm26W4Q"
}
"##,
            ),
            None,
        ),
        Endpoint::SharingUnmountFolderPost => (
            Some(
                r##"{
    "shared_folder_id": "84528192421"
}"##,
            ),
            None,
        ),
        Endpoint::SharingUnshareFilePost => (
            Some(
                r##"{
    "file": "id:3kmLmQFnf1AAAAAAAAAAAw"
}
"##,
            ),
            None,
        ),
        Endpoint::SharingUnshareFolderPost => (
            Some(
                r##"{
    "leave_a_copy": false,
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
    ".tag": "complete"
}"##,
            ),
        ),
        Endpoint::SharingUpdateFileMemberPost => (
            Some(
                r##"{
    "access_level": "viewer",
    "file": "id:3kmLmQFnf1AAAAAAAAAAAw",
    "member": {
        ".tag": "email",
        "email": "justin@example.com"
    }
}"##,
            ),
            Some(
                r##"{
  "access_level": "viewer",
  "warning": "The user has limited access due to organizational policy.",
  "access_details": [
    {
      "parent_folder_id": "parent1234567890abcdef",
      "parent_folder_name": "Company Policies",
      "access_level": "viewer"
    },
    {
      "parent_folder_id": "parentabcdef1234567890",
      "parent_folder_name": "HR Documents",
      "access_level": "editor"
    }
  ]
}
"##,
            ),
        ),
        Endpoint::SharingUpdateFolderMemberPost => (
            Some(
                r##"{
    "access_level": "editor",
    "member": {
        ".tag": "email",
        "email": "justin@example.com"
    },
    "shared_folder_id": "84528192421"
}"##,
            ),
            Some(
                r##"{
  "access_level": "editor",
  "warning": "The user has access to the content through a shared team folder.",
  "access_details": [
    {
      "parent_folder_id": "folder1234567890abcdef",
      "parent_folder_name": "Shared Team Folder",
      "access_level": "editor"
    },
    {
      "parent_folder_id": "folderabcdef1234567890",
      "parent_folder_name": "Project Documents",
      "access_level": "viewer"
    }
  ]
}"##,
            ),
        ),
        Endpoint::SharingUpdateFolderPolicyPost => (
            Some(
                r##"{
    "acl_update_policy": "owner",
    "member_policy": "team",
    "shared_folder_id": "84528192421",
    "shared_link_policy": "members"
}"##,
            ),
            Some(
                r##"{
    "access_inheritance": {
        ".tag": "inherit"
    },
    "access_type": {
        ".tag": "owner"
    },
    "folder_id": "id:a4ayc_80_OEAAAAAAAAAXw",
    "is_inside_team_folder": false,
    "is_team_folder": false,
    "link_metadata": {
        "audience_options": [
            {
                ".tag": "public"
            },
            {
                ".tag": "team"
            },
            {
                ".tag": "members"
            }
        ],
        "current_audience": {
            ".tag": "public"
        },
        "link_permissions": [
            {
                "action": {
                    ".tag": "change_audience"
                },
                "allow": true
            }
        ],
        "password_protected": false,
        "url": ""
    },
    "name": "dir",
    "path_lower": "/dir",
    "permissions": [],
    "policy": {
        "acl_update_policy": {
            ".tag": "owner"
        },
        "member_policy": {
            ".tag": "anyone"
        },
        "resolved_member_policy": {
            ".tag": "team"
        },
        "shared_link_policy": {
            ".tag": "anyone"
        }
    },
    "preview_url": "https://www.dropbox.com/scl/fo/fir9vjelf",
    "shared_folder_id": "84528192421",
    "time_invited": "2016-01-20T00:00:00Z"
}"##,
            ),
        ),
        Endpoint::UsersFeaturesGetValuesPost => (
            Some(
                r##"{
    "features": [
        {
            ".tag": "paper_as_files"
        },
        {
            ".tag": "file_locking"
        }
    ]
}"##,
            ),
            Some(
                r##"{
    "values": [
        {
            ".tag": "paper_as_files",
            "paper_as_files": {
                ".tag": "enabled",
                "enabled": true
            }
        }
    ]
}"##,
            ),
        ),
        Endpoint::UsersGetAccountPost => (
            Some(
                r##"{
    "account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc"
}"##,
            ),
            Some(
                r##"
{
    "account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
    "disabled": false,
    "email": "franz@dropbox.com",
    "email_verified": true,
    "is_teammate": false,
    "name": {
        "abbreviated_name": "FF",
        "display_name": "Franz Ferdinand (Personal)",
        "familiar_name": "Franz",
        "given_name": "Franz",
        "surname": "Ferdinand"
    },
    "profile_photo_url": "https://dl-web.dropbox.com/account_photo/get/dbaphid%3AAAHWGmIXV3sUuOmBfTz0wPsiqHUpBWvv3ZA?vers=1556069330102&size=128x128"
}"##,
            ),
        ),
        Endpoint::UsersGetAccountBatchPost => (
            Some(
                r##"{
    "account_ids": [
        "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
        "dbid:AAH1Vcz-DVoRDeixtr_OA8oUGgiqhs4XPOQ"
    ]
}"##,
            ),
            Some(
                r##"{
  "accounts": [
    {
      "account_id": "1234567890abcdefghijklmnopqrstuvwxyzabcd",
      "name": {
        "given_name": "John",
        "surname": "Doe",
        "familiar_name": "John",
        "display_name": "John Doe",
        "abbreviated_name": "JD"
      },
      "email": "john.doe@example.com",
      "email_verified": true,
      "disabled": false,
      "is_teammate": true,
      "profile_photo_url": "https://example.com/profile_photos/johndoe.jpg",
      "team_member_id": "team1234567890abcdef"
    },
    {
      "account_id": "abcdefghijklmnopqrstuvwxyzabcdef1234567890",
      "name": {
        "given_name": "Jane",
        "surname": "Smith",
        "familiar_name": "Jane",
        "display_name": "Jane Smith",
        "abbreviated_name": "JS"
      },
      "email": "jane.smith@example.com",
      "email_verified": false,
      "disabled": false,
      "is_teammate": false,
      "profile_photo_url": null,
      "team_member_id": null
    },
    {
      "account_id": "abcdef1234567890abcdefghijklmnopqrstuvwxyz",
      "name": {
        "given_name": "Alice",
        "surname": "Johnson",
        "familiar_name": "Alice",
        "display_name": "Alice Johnson",
        "abbreviated_name": "AJ"
      },
      "email": "alice.johnson@example.com",
      "email_verified": true,
      "disabled": true,
      "is_teammate": true,
      "profile_photo_url": "https://example.com/profile_photos/alicejohnson.jpg",
      "team_member_id": "teamabcdef1234567890"
    }
  ]
}
"##,
            ),
        ),
        Endpoint::UsersGetCurrentAccountPost => (
            None,
            Some(
                r##"{
    "account_id": "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc",
    "account_type": {
        ".tag": "business"
    },
    "country": "US",
    "disabled": false,
    "email": "franz@dropbox.com",
    "email_verified": true,
    "is_paired": true,
    "locale": "en",
    "name": {
        "abbreviated_name": "FF",
        "display_name": "Franz Ferdinand (Personal)",
        "familiar_name": "Franz",
        "given_name": "Franz",
        "surname": "Ferdinand"
    },
    "referral_link": "https://db.tt/ZITNuhtI",
    "root_info": {
        ".tag": "user",
        "home_namespace_id": "3235641",
        "root_namespace_id": "3235641"
    },
    "team": {
        "id": "dbtid:AAFdgehTzw7WlXhZJsbGCLePe8RvQGYDr-I",
        "name": "Acme, Inc.",
        "office_addin_policy": {
            ".tag": "disabled"
        },
        "sharing_policies": {
            "default_link_expiration_days_policy": {
                ".tag": "none"
            },
            "enforce_link_password_policy": {
                ".tag": "optional"
            },
            "group_creation_policy": {
                ".tag": "admins_only"
            },
            "shared_folder_join_policy": {
                ".tag": "from_anyone"
            },
            "shared_folder_link_restriction_policy": {
                ".tag": "anyone"
            },
            "shared_folder_member_policy": {
                ".tag": "team"
            },
            "shared_link_create_policy": {
                ".tag": "team_only"
            },
            "shared_link_default_permissions_policy": {
                ".tag": "default"
            }
        },
        "top_level_content_policy": {
            ".tag": "admin_only"
        }
    },
    "team_member_id": "dbmid:AAHhy7WsR0x-u4ZCqiDl5Fz5zvuL3kmspwU"
}"##,
            ),
        ),
        Endpoint::UsersGetSpaceUsagePost => (
            None,
            Some(
                r##"{
    "allocation": {
        ".tag": "individual",
        "allocated": 10000000000
    },
    "used": 314159265
}"##,
            ),
        ),
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
