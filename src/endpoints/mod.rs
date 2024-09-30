pub mod headers;
use super::tests_utils::{
    MOCK_SERVER_ASYNC_PORT, MOCK_SERVER_ASYNC_URL, MOCK_SERVER_SYNC_PORT, MOCK_SERVER_SYNC_URL,
};

/// Enum representing api available endpoints
/// It is passed to fhe function
#[derive(Debug)]
pub enum Endpoint {
    CheckAppPost,
    CheckUserPost,
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

#[allow(unused_variables)]
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
        Endpoint::FileRequestsDeletePost => "https://api.dropboxapi.com/2/file_requests/delete",
        Endpoint::FilesCopyPost => "https://api.dropboxapi.com/2/files/copy_v2",
        Endpoint::FilesCopyBatchPost => "https://api.dropboxapi.com/2/files/copy_batch_v2",
        Endpoint::FilesCopyBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/copy_batch/check_v2"
        }
        Endpoint::FilesCopyReferenceGetPost => {
            "https://api.dropboxapi.com/2/files/copy_reference/get"
        }
        Endpoint::FilesCopyReferenceSavePost => {
            "https://api.dropboxapi.com/2/files/copy_reference/save"
        }
        Endpoint::FilesCreateFolderPost => "https://api.dropboxapi.com/2/files/create_folder_v2",
        Endpoint::FilesCreateFolderBatchPost => {
            "https://api.dropboxapi.com/2/files/create_folder_batch"
        }
        Endpoint::FilesCreateFolderBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/create_folder_batch/check"
        }
        Endpoint::FilesDeleteBatchPost => "https://api.dropboxapi.com/2/files/delete_batch",
        Endpoint::FilesDeleteBatchCheckPost => {
            "https://api.dropboxapi.com/2/files/delete_batch/check"
        }
        Endpoint::FilesDownloadPost => "https://content.dropboxapi.com/2/files/download",
        Endpoint::FilesDownloadZipPost => "https://content.dropboxapi.com/2/files/download_zip",
        Endpoint::FilesExportPost => "https://content.dropboxapi.com/2/files/export",
        Endpoint::FilesGetFileLockBatchPost => {
            "https://api.dropboxapi.com/2/files/get_file_lock_batch"
        }
        Endpoint::FilesGetMetadataPost => "https://api.dropboxapi.com/2/files/get_metadata",
        Endpoint::FilesGetPreviewPost => "https://content.dropboxapi.com/2/files/get_preview",
        Endpoint::FilesGetTemporaryLinkPost => {
            "https://api.dropboxapi.com/2/files/get_temporary_link"
        }
        Endpoint::FilesGetTemporaryUploadLinkPost => {
            "https://api.dropboxapi.com/2/files/get_temporary_upload_link"
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
            "https://api.dropboxapi.com/2/files/list_folder/get_latest_cursor"
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
        Endpoint::FilesTagsRemovePost => "https://api.dropboxapi.com/2/files/tags/remove",
        Endpoint::FilesUnlockFileBatchPost => {
            "https://api.dropboxapi.com/2/files/unlock_file_batch"
        }
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
            "https://api.dropboxapi.com/2/sharing/update_folder_policy"
        }
    };

    let binding: (String, Option<String>, Option<String>) = (url.to_string(), None, None);
    #[cfg(feature = "test-utils")]
    let binding = test_url(url);

    binding
}

/// For testing purpose, it will replace original end-point with mock server url
#[allow(unused)]
fn test_url(url: &str) -> (String, Option<String>, Option<String>) {
    let idx = url.find("com").expect("should have com") + 3;

    let url_test_sync = format!(
        "http://{}:{}{}",
        MOCK_SERVER_SYNC_URL,
        MOCK_SERVER_SYNC_PORT,
        &url[idx..]
    );
    let url_test_async = format!(
        "http://{}:{}{}",
        MOCK_SERVER_ASYNC_URL,
        MOCK_SERVER_ASYNC_PORT,
        &url[idx..]
    );
    (url.to_string(), Some(url_test_sync), Some(url_test_async))
}
