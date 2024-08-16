mod add_file_member;
mod add_folder_member;
mod check_job_status;
mod create_shared_link_with_settings;
mod get_file_metadata;
mod get_file_metadata_batch;
mod get_shared_link_metadata;
mod list_file_members;
mod list_file_members_batch;
mod list_file_members_continue;
mod list_folder_members;
mod list_folder_members_continue;
mod list_folders;
mod list_folders_continue;
mod list_mountable_folders;
mod list_mountable_folders_continue;
mod list_received_files;
mod list_received_files_continue;
mod list_shared_links;
mod modify_shared_link_settings;
mod mount_folder;
mod relinquish_file_membership;
mod remove_file_member_2;
mod remove_folder_member;
mod revoke_shared_link;
mod share_folder;
mod transfer_folder;
mod unmount_folder;
mod unshare_file;
mod unshare_folder;
mod update_file_member;
mod update_folder_policy;

use serde::{Deserialize, Serialize};

//////////////////// Common Enums and Structs ////////////////////

#[derive(Serialize, Deserialize)]
pub enum AccessLevel {
    #[serde(rename = "viewer")]
    Viewer,
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "no_access")]
    NoAccess,
    #[serde(rename = "viewer_no_comment")]
    ViewerNoComment,
    #[serde(rename = "traverse")]
    Traverse,
}

#[derive(Serialize, Deserialize)]
pub enum ACLUpdatePolicy {
    #[serde(rename = "editors")]
    Editors,
    #[serde(rename = "owner")]
    Owner,
}

#[derive(Serialize, Deserialize)]
pub enum MemberPolicy {
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "anyone")]
    Anyone,
}

#[derive(Serialize, Deserialize)]
pub enum SharedLinkPolicy {
    #[serde(rename = "anyone")]
    Anyone,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "members")]
    Members,
}

#[derive(Serialize, Deserialize)]
pub enum AccessInheritance {
    #[serde(rename = "inherit")]
    Inherit,
    #[serde(rename = "no_inherit")]
    NoInherit,
}

#[derive(Serialize, Deserialize)]
pub enum JobStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorTag {
    // General Error Types
    RateLimit,
    InvalidComment,
    BannedMember,
    InvalidAsyncJobId,
    InternalError,
    Other,
    UserError,
    AccessError,
    NoPermission,
    EmailUnverified,

    // Specific Errors for Endpoints:
    InvalidSharedFolder,
    InvalidMember,
    NoExplicitAccess,
    SharedLinkNotFound,
    SharedLinkAccessDenied,
    UnsupportedLinkType,
    UnsupportedParameterField,
    SharedLinkMalformed,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = ".tag")]
    pub tag: ErrorTag,
    pub error_summary: String,
}

/////////////////////////////////////////////////////////////
////////////////// All 38 Endpoints /////////////////////////
/////////////////////////////////////////////////////////////

// 1. Add File Member
#[derive(Serialize, Deserialize)]
pub struct AddFileMemberArgs {
    pub file: String,
    pub members: Vec<MemberSelector>,
    pub custom_message: Option<String>,
    pub quiet: Option<bool>,
    pub access_level: Option<AccessLevel>,
    pub add_message_as_comment: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FileMemberActionResult {
    pub member: MemberSelector,
    pub result: FileMemberActionResultTag,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileMemberActionResultTag {
    Success,
    MemberError,
}

#[derive(Serialize, Deserialize)]
pub struct AddFileMemberError {
    #[serde(rename = ".tag")]
    pub tag: ErrorTag,
    pub user_error: Option<String>,
}

////////////////////////////////////////////////////////////

// 2. Add Folder Member
#[derive(Serialize, Deserialize)]
pub struct AddFolderMemberArgs {
    pub shared_folder_id: String,
    pub members: Vec<AddMember>,
    pub quiet: Option<bool>,
    pub custom_message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AddMember {
    pub member: MemberSelector,
    pub access_level: AccessLevel,
}

#[derive(Serialize, Deserialize)]
pub struct AddFolderMemberError {
    #[serde(rename = ".tag")]
    pub tag: ErrorTag,
}

////////////////////////////////////////////////////////////

// 3. Check Job Status
#[derive(Serialize, Deserialize)]
pub struct PollArg {
    pub async_job_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct JobStatusResult {
    #[serde(rename = ".tag")]
    pub status: JobStatus,
}

////////////////////////////////////////////////////////////

// 4. Create Shared Link with Settings
#[derive(Serialize, Deserialize)]
pub struct CreateSharedLinkWithSettingsArgs {
    pub path: String,
    pub settings: Option<LinkSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkSettings {
    pub access: Option<AccessLevel>,
    pub allow_download: Option<bool>,
    pub audience: Option<String>,
    pub requested_visibility: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SharedLinkMetadata {
    pub url: String,
    pub id: String,
    pub name: String,
    pub path_lower: Option<String>,
    pub team_member_info: Option<TeamMemberInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamMemberInfo {
    pub display_name: String,
}

////////////////////////////////////////////////////////////

// 5. Get File Metadata
#[derive(Serialize, Deserialize)]
pub struct GetFileMetadataArgs {
    pub file: String,
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetFileMetadataResult {
    pub id: String,
    pub name: String,
    pub access_type: Option<AccessLevel>,
    pub policy: Option<FolderPolicy>,
}

////////////////////////////////////////////////////////////

// 6. List File Members
#[derive(Serialize, Deserialize)]
pub struct ListFileMembersArgs {
    pub file: String,
    pub include_inherited: Option<bool>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SharedFileMembers {
    pub users: Vec<UserFileMembershipInfo>,
    pub groups: Vec<GroupMembershipInfo>,
    pub invitees: Vec<InviteeMembershipInfo>,
    pub cursor: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserFileMembershipInfo {
    pub access_type: AccessLevel,
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize)]
pub struct GroupMembershipInfo {
    pub access_type: AccessLevel,
}

#[derive(Serialize, Deserialize)]
pub struct InviteeMembershipInfo {
    pub access_type: AccessLevel,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub account_id: String,
    pub email: String,
    pub name: String,
}

////////////////////////////////////////////////////////////

// 7. Mount Folder
#[derive(Serialize, Deserialize)]
pub struct MountFolderArgs {
    pub shared_folder_id: String,
}

////////////////////////////////////////////////////////////

// 8. Relinquish File Membership
#[derive(Serialize, Deserialize)]
pub struct RelinquishFileMembershipArgs {
    pub file: String,
}

////////////////////////////////////////////////////////////

// 9. Remove File Member
#[derive(Serialize, Deserialize)]
pub struct RemoveFileMemberArgs {
    pub file: String,
    pub member: MemberSelector,
}

////////////////////////////////////////////////////////////

// 10. Remove Folder Member
#[derive(Serialize, Deserialize)]
pub struct RemoveFolderMemberArgs {
    pub shared_folder_id: String,
    pub member: MemberSelector,
    pub leave_a_copy: bool,
}

////////////////////////////////////////////////////////////

// 11. Revoke Shared Link
#[derive(Serialize, Deserialize)]
pub struct RevokeSharedLinkArgs {
    pub url: String,
}

////////////////////////////////////////////////////////////

// 12. Share Folder
#[derive(Serialize, Deserialize)]
pub struct ShareFolderArgs {
    pub path: String,
    pub acl_update_policy: Option<ACLUpdatePolicy>,
    pub member_policy: Option<MemberPolicy>,
    pub shared_link_policy: Option<SharedLinkPolicy>,
    pub force_async: Option<bool>,
    pub access_inheritance: Option<AccessInheritance>,
}

////////////////////////////////////////////////////////////

// 13. Transfer Folder
#[derive(Serialize, Deserialize)]
pub struct TransferFolderArgs {
    pub shared_folder_id: String,
    pub to_dropbox_id: String,
}

////////////////////////////////////////////////////////////

// 14. Unmount Folder
#[derive(Serialize, Deserialize)]
pub struct UnmountFolderArgs {
    pub shared_folder_id: String,
}

////////////////////////////////////////////////////////////

// 15. Unshare File
#[derive(Serialize, Deserialize)]
pub struct UnshareFileArgs {
    pub file: String,
}

////////////////////////////////////////////////////////////

// 16. Unshare Folder
#[derive(Serialize, Deserialize)]
pub struct UnshareFolderArgs {
    pub shared_folder_id: String,
    pub leave_a_copy: bool,
}

////////////////////////////////////////////////////////////

// 17. Update File Member
#[derive(Serialize, Deserialize)]
pub struct UpdateFileMemberArgs {
    pub file: String,
    pub member: MemberSelector,
    pub access_level: AccessLevel,
}

////////////////////////////////////////////////////////////

// 18. Update Folder Member
#[derive(Serialize, Deserialize)]
pub struct UpdateFolderMemberArgs {
    pub shared_folder_id: String,
    pub member: MemberSelector,
    pub access_level: AccessLevel,
}

////////////////////////////////////////////////////////////

// 19. Update Folder Policy
#[derive(Serialize, Deserialize)]
pub struct UpdateFolderPolicyArgs {
    pub shared_folder_id: String,
    pub member_policy: Option<MemberPolicy>,
    pub acl_update_policy: Option<ACLUpdatePolicy>,
    pub shared_link_policy: Option<SharedLinkPolicy>,
}

////////////////////////////////////////////////////////////

// 20. List Folder Members
#[derive(Serialize, Deserialize)]
pub struct ListFolderMembersArgs {
    pub shared_folder_id: String,
    pub actions: Option<Vec<String>>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SharedFolderMembers {
    pub users: Vec<UserFileMembershipInfo>,
    pub groups: Vec<GroupMembershipInfo>,
    pub invitees: Vec<InviteeMembershipInfo>,
    pub cursor: Option<String>,
}

////////////////////////////////////////////////////////////

// 21. List File Members Batch
#[derive(Serialize, Deserialize)]
pub struct ListFileMembersBatchArgs {
    pub files: Vec<String>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ListFileMembersBatchResult {
    pub file: String,
    pub result: SharedFileMembers,
}

////////////////////////////////////////////////////////////

// 22. List File Members Continue
#[derive(Serialize, Deserialize)]
pub struct ListFileMembersContinueArgs {
    pub cursor: String,
}

////////////////////////////////////////////////////////////

// 23. List Folder Members Continue
#[derive(Serialize, Deserialize)]
pub struct ListFolderMembersContinueArgs {
    pub cursor: String,
}

////////////////////////////////////////////////////////////

// 24. List Folders
#[derive(Serialize, Deserialize)]
pub struct ListFoldersArgs {
    pub limit: Option<u32>,
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ListFoldersResult {
    pub entries: Vec<SharedFolderMetadata>,
    pub cursor: Option<String>,
}

////////////////////////////////////////////////////////////

// 25. List Folders Continue
#[derive(Serialize, Deserialize)]
pub struct ListFoldersContinueArgs {
    pub cursor: String,
}

////////////////////////////////////////////////////////////

// 26. List Mountable Folders
#[derive(Serialize, Deserialize)]
pub struct ListMountableFoldersArgs {
    pub limit: Option<u32>,
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ListMountableFoldersResult {
    pub entries: Vec<SharedFolderMetadata>,
    pub cursor: Option<String>,
}

////////////////////////////////////////////////////////////

// 27. List Mountable Folders Continue
#[derive(Serialize, Deserialize)]
pub struct ListMountableFoldersContinueArgs {
    pub cursor: String,
}

////////////////////////////////////////////////////////////

// 28. List Received Files
#[derive(Serialize, Deserialize)]
pub struct ListReceivedFilesArgs {
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ListReceivedFilesResult {
    pub entries: Vec<SharedFileMetadata>,
    pub cursor: Option<String>,
}

////////////////////////////////////////////////////////////

// 29. List Received Files Continue
#[derive(Serialize, Deserialize)]
pub struct ListReceivedFilesContinueArgs {
    pub cursor: String,
}

////////////////////////////////////////////////////////////

// 30. List Shared Links
#[derive(Serialize, Deserialize)]
pub struct ListSharedLinksArgs {
    pub path: Option<String>,
    pub cursor: Option<String>,
    pub direct_only: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ListSharedLinksResult {
    pub links: Vec<SharedLinkMetadata>,
    pub cursor: Option<String>,
    pub has_more: bool,
}

////////////////////////////////////////////////////////////

// 31. Modify Shared Link Settings
#[derive(Serialize, Deserialize)]
pub struct ModifySharedLinkSettingsArgs {
    pub url: String,
    pub settings: LinkSettings,
    pub remove_expiration: Option<bool>,
}

////////////////////////////////////////////////////////////

// 32. Get Shared Link Metadata
#[derive(Serialize, Deserialize)]
pub struct GetSharedLinkMetadataArgs {
    pub url: String,
    pub path: Option<String>,
    pub link_password: Option<String>,
}

////////////////////////////////////////////////////////////

// 33. Get File Metadata Batch
#[derive(Serialize, Deserialize)]
pub struct GetFileMetadataBatchArgs {
    pub files: Vec<String>,
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetFileMetadataBatchResult {
    pub entries: Vec<GetFileMetadataResult>,
}

////////////////////////////////////////////////////////////

// 34. Unshare Folder Error
#[derive(Serialize, Deserialize)]
pub enum UnshareFolderError {
    TeamFolder,
    NoPermission,
    TooManyFiles,
    Other,
}

////////////////////////////////////////////////////////////

// 35. Remove File Membership Error
#[derive(Serialize, Deserialize)]
pub enum RemoveFileMembershipError {
    FolderOwner,
    GroupAccess,
    TeamFolder,
    NoPermission,
    TooManyFiles,
    NoExplicitAccess,
    NoAccess,
}

////////////////////////////////////////////////////////////

// 36. FolderMemberActionError
#[derive(Serialize, Deserialize)]
pub enum FolderMemberActionError {
    NoPermission,
    AccessError,
    MemberError,
}

////////////////////////////////////////////////////////////

// 37. Revoke Shared Link Error
#[derive(Serialize, Deserialize)]
pub enum RevokeSharedLinkError {
    SharedLinkNotFound,
    SharedLinkAccessDenied,
    UnsupportedLinkType,
    UnsupportedParameterField,
    SharedLinkMalformed,
}

////////////////////////////////////////////////////////////

// 38. Transfer Folder Error
#[derive(Serialize, Deserialize)]
pub enum TransferFolderError {
    InvalidDropboxId,
    NoPermission,
    NewOwnerNotAMember,
    NewOwnerUnmounted,
    NewOwnerEmailUnverified,
}

////////////////////////////////////////////////////////////

// Member Selector

#[derive(Serialize, Deserialize)]
pub struct MemberSelector {
    #[serde(rename = ".tag")]
    pub tag: String,
    pub email: Option<String>,
    pub dropbox_id: Option<String>,
}

// Folder Policy

#[derive(Serialize, Deserialize)]
pub struct FolderPolicy {
    pub acl_update_policy: Option<ACLUpdatePolicy>,
    pub shared_link_policy: Option<SharedLinkPolicy>,
    pub member_policy: Option<MemberPolicy>,
    pub resolved_member_policy: Option<MemberPolicy>,
}

// SharedFolderMetadata

#[derive(Serialize, Deserialize)]
pub struct SharedFolderMetadata {
    pub shared_folder_id: String,
    pub path_lower: String,
    pub name: String,
    pub access_inheritance: AccessInheritance,
    pub policy: FolderPolicy,
}

// SharedFileMetadata

#[derive(Serialize, Deserialize)]
pub struct SharedFileMetadata {
    pub access_type: AccessLevel,
    pub id: String,
    pub name: String,
    pub owner_display_names: Vec<String>,
    pub path_display: Option<String>,
}
