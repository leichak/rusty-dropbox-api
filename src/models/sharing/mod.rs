//! Types and endpoints for the Dropbox `sharing` namespace.
//!
//! All 39 endpoints are wired. Request/Response payloads are typed as
//! `serde_json::Value` for most endpoints in this initial pass so the
//! namespace is reachable end-to-end; full Rust struct coverage of
//! `SharedLinkMetadata`, member-policy types, and folder-share types is a
//! follow-up ratchet (same methodology as the files struct audit).
//!
//! Two endpoints ship with typed args already (`revoke_shared_link`,
//! `list_shared_links`) — those types live in this module.
//!
//! Reference: <https://www.dropbox.com/developers/documentation/http/documentation#sharing>

pub mod add_file_member;
pub mod add_folder_member;
pub mod check_job_status;
pub mod check_remove_member_job_status;
pub mod check_share_job_status;
pub mod create_shared_link_with_settings;
pub mod get_file_metadata;
pub mod get_file_metadata_batch;
pub mod get_folder_metadata;
pub mod get_shared_link_file;
pub mod get_shared_link_metadata;
pub mod list_file_members;
pub mod list_file_members_batch;
pub mod list_file_members_continue;
pub mod list_folder_members;
pub mod list_folder_members_continue;
pub mod list_folders;
pub mod list_folders_continue;
pub mod list_mountable_folders;
pub mod list_mountable_folders_continue;
pub mod list_received_files;
pub mod list_received_files_continue;
pub mod list_shared_links;
pub mod modify_shared_link_settings;
pub mod mount_folder;
pub mod relinquish_file_membership;
pub mod relinquish_folder_membership;
pub mod remove_file_member_2;
pub mod remove_folder_member;
pub mod revoke_shared_link;
pub mod set_access_inheritance;
pub mod share_folder;
pub mod transfer_folder;
pub mod unmount_folder;
pub mod unshare_file;
pub mod unshare_folder;
pub mod update_file_member;
pub mod update_folder_member;
pub mod update_folder_policy;

use serde::{Deserialize, Serialize};

/// Argument to `revoke_shared_link`.
#[derive(Serialize, Deserialize, Debug)]
pub struct RevokeSharedLinkArg {
    pub url: String,
}

/// Argument to `list_shared_links`.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListSharedLinksArg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_only: Option<bool>,
}

/// Result of `list_shared_links`.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListSharedLinksResult {
    pub links: Vec<SharedLinkMetadata>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// ---- Shared-link metadata tree ----

/// Top-level SharedLinkMetadata. Dropbox flattens file/folder-specific fields
/// at the same level as `.tag`, so we use struct variants with `#[serde(rename)]`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharedLinkMetadata {
    File(FileLinkMetadata),
    Folder(FolderLinkMetadata),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileLinkMetadata {
    pub url: String,
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_lower: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_modified: Option<String>,
    /// The link permission tree is deeply nested — kept loosely typed for now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_permissions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_owner_team_info: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderLinkMetadata {
    pub url: String,
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_lower: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_permissions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_info: Option<serde_json::Value>,
}

// ---- create_shared_link_with_settings ----

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSharedLinkWithSettingsArg {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SharedLinkSettings>,
}

/// Optional settings when creating a shared link. All fields optional; any
/// unspecified key defaults to Dropbox's per-account policy.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SharedLinkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
}

// ---- get_shared_link_metadata ----

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSharedLinkMetadataArg {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_password: Option<String>,
}

// ---- modify_shared_link_settings ----

#[derive(Serialize, Deserialize, Debug)]
pub struct ModifySharedLinkSettingsArgs {
    pub url: String,
    pub settings: SharedLinkSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_expiration: Option<bool>,
}

// ---- shared folder Arg/cursor types (responses kept as serde_json::Value) ----

/// A shared_folder_id-only request (used by mount, unmount, unshare, transfer,
/// relinquish_folder_membership and similar simple folder ops).
#[derive(Serialize, Deserialize, Debug)]
pub struct SharedFolderIdArg {
    pub shared_folder_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareFolderArg {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_update_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_async: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_link_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_info_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_inheritance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_settings: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnshareFolderArg {
    pub shared_folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_a_copy: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFolderArg {
    pub shared_folder_id: String,
    pub to_dropbox_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFolderPolicyArg {
    pub shared_folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_update_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_info_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_link_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAccessInheritanceArg {
    pub shared_folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_inheritance: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListFoldersArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFoldersContinueArg {
    pub cursor: String,
}

// ---- file-membership Arg types ----

#[derive(Serialize, Deserialize, Debug)]
pub struct UnshareFileArg {
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileMetadataArg {
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileMetadataBatchArg {
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFolderMetadataArg {
    pub shared_folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileMembersArg {
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inherited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileMembersBatchArg {
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileMembersContinueArg {
    pub cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderMembersArgs {
    pub shared_folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderMembersContinueArg {
    pub cursor: String,
}

/// PollArg-shaped argument used by `check_*_job_status` endpoints.
#[derive(Serialize, Deserialize, Debug)]
pub struct PollArg {
    pub async_job_id: String,
}

// ---- member selectors and Args ----

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MemberSelector {
    DropboxId(String),
    Email(String),
}

/// `members` is a list of `{member, custom_message?, ...}` entries; we keep
/// individual members as Value to avoid pulling in InviteeInfo etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct AddFileMemberArgs {
    pub file: String,
    pub members: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_message_as_comment: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFolderMemberArg {
    pub shared_folder_id: String,
    pub members: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveFileMemberArg {
    pub file: String,
    pub member: MemberSelector,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveFolderMemberArg {
    pub shared_folder_id: String,
    pub member: MemberSelector,
    pub leave_a_copy: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFileMemberArgs {
    pub file: String,
    pub member: MemberSelector,
    pub access_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFolderMemberArg {
    pub shared_folder_id: String,
    pub member: MemberSelector,
    pub access_level: String,
}
