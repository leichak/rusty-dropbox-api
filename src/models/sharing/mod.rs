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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_permissions: Option<LinkPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_info: Option<TeamMemberInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_owner_team_info: Option<Team>,
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
    pub link_permissions: Option<LinkPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_info: Option<TeamMemberInfo>,
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
///
/// Field set verified against the Stone spec at
/// `dropbox-api-spec/shared_links.stone`.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SharedLinkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<LinkAudience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<RequestedLinkAccessLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_visibility: Option<RequestedVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum RequestedVisibility {
    Public,
    TeamOnly,
    Password,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum LinkAudience {
    Public,
    Team,
    NoOne,
    Password,
    Members,
    Other,
}

/// Per Stone spec `shared_links.stone`, `LinkAccessLevel` has only two
/// variants. `Other` is added defensively so future Dropbox additions don't
/// break deserialization.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum LinkAccessLevel {
    Viewer,
    Editor,
    Other,
}

/// Used by `SharedLinkSettings.access` (request-side; the spec calls it
/// `RequestedLinkAccessLevel` and adds `max` + `default` to the response-side
/// `LinkAccessLevel`).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum RequestedLinkAccessLevel {
    Viewer,
    Editor,
    Max,
    Default,
    Other,
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
    pub acl_update_policy: Option<AclUpdatePolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_async: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_policy: Option<MemberPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_link_policy: Option<SharedLinkPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_info_policy: Option<ViewerInfoPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_inheritance: Option<AccessInheritance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<FolderAction>>,
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
    pub member_policy: Option<MemberPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_update_policy: Option<AclUpdatePolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_info_policy: Option<ViewerInfoPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_link_policy: Option<SharedLinkPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<FolderAction>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAccessInheritanceArg {
    pub shared_folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_inheritance: Option<AccessInheritance>,
}

/// Per Stone spec `RelinquishFolderMembershipArg` has its own shape.
/// Previously this endpoint reused `SharedFolderIdArg` — that worked but
/// dropped the `leave_a_copy` flag.
#[derive(Serialize, Deserialize, Debug)]
pub struct RelinquishFolderMembershipArg {
    pub shared_folder_id: String,
    #[serde(default)]
    pub leave_a_copy: bool,
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
    pub actions: Option<Vec<MemberAction>>,
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
    pub actions: Option<Vec<FolderAction>>,
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

/// Stone: `union MemberSelector { dropbox_id String | email String }`.
/// Wire form: `{".tag": "email", "email": "..."}`. Modelled as struct
/// variants — internally tagged enums + tuple variants of String don't work
/// in serde.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MemberSelector {
    DropboxId { dropbox_id: String },
    Email { email: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFileMemberArgs {
    pub file: String,
    pub members: Vec<MemberSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_message_as_comment: Option<bool>,
}

/// Single entry in `AddFolderMemberArg.members`. Per Stone spec:
///   member: MemberSelector
///   access_level: AccessLevel = viewer
#[derive(Serialize, Deserialize, Debug)]
pub struct AddMember {
    pub member: MemberSelector,
    #[serde(default = "default_viewer_access_level")]
    pub access_level: AccessLevel,
}

fn default_viewer_access_level() -> AccessLevel {
    AccessLevel::Viewer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFolderMemberArg {
    pub shared_folder_id: String,
    pub members: Vec<AddMember>,
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
    pub access_level: AccessLevel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFolderMemberArg {
    pub shared_folder_id: String,
    pub member: MemberSelector,
    pub access_level: AccessLevel,
}

// ---- typed response wrappers (deeply nested fields stay as Value) ----

/// Result of `list_folders` / `list_mountable_folders`.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListFoldersResult {
    pub entries: Vec<SharedFolderMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Result of `list_received_files`.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListReceivedFilesResult {
    pub entries: Vec<SharedFileMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Result of `list_file_members`.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileMembersResult {
    pub users: Vec<UserMembershipInfo>,
    pub groups: Vec<GroupMembershipInfo>,
    pub invitees: Vec<InviteeMembershipInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Result of `list_folder_members`.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderMembersResult {
    pub users: Vec<UserMembershipInfo>,
    pub groups: Vec<GroupMembershipInfo>,
    pub invitees: Vec<InviteeMembershipInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Launch envelope for `share_folder`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ShareFolderLaunch {
    AsyncJobId { async_job_id: String },
    Complete(serde_json::Value),
}

/// Launch envelope for `unshare_folder` and similar fire-and-forget folder ops.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum LaunchEmptyResult {
    AsyncJobId { async_job_id: String },
    Complete,
}

/// Job status for `check_share_job_status`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ShareFolderJobStatus {
    InProgress,
    Complete(serde_json::Value),
    Failed(serde_json::Value),
}

/// Job status for `check_remove_member_job_status`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum RemoveMemberJobStatus {
    InProgress,
    Complete(serde_json::Value),
    Failed(serde_json::Value),
}

/// Job status for `check_job_status` (generic).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum JobStatus {
    InProgress,
    Complete,
    Failed(serde_json::Value),
}

// =============================================================================
// Link permission tree — fills the `link_permissions` field on SharedLinkMetadata
// =============================================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_visibility: Option<ResolvedVisibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_visibility: Option<RequestedVisibility>,
    #[serde(default)]
    pub can_revoke: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke_failure_reason: Option<SharedLinkAccessFailureReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_audience: Option<LinkAudience>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_access_level: Option<LinkAccessLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_policies: Option<Vec<VisibilityPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_set_expiry: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_remove_expiry: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_allow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_disallow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_comments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_restricts_comments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience_options: Option<Vec<LinkAudienceOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_set_password: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_remove_password: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_password: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_use_extended_sharing_controls: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ResolvedVisibility {
    Public,
    TeamOnly,
    Password,
    TeamAndPassword,
    SharedFolderOnly,
    NoOne,
    OnlyYou,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharedLinkAccessFailureReason {
    LoginRequired,
    EmailVerifyRequired,
    PasswordRequired,
    TeamOnly,
    OwnerOnly,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VisibilityPolicy {
    pub policy: RequestedVisibility,
    pub resolved_policy: ResolvedVisibility,
    pub allowed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disallowed_reason: Option<SharedLinkAccessFailureReason>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkAudienceOption {
    pub audience: LinkAudience,
    pub allowed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disallowed_reason: Option<LinkAudienceDisallowedReason>,
}

// =============================================================================
// Team / member info types
// =============================================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMemberInfo {
    pub team_info: Team,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub account_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub same_team: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupInfo {
    pub group_name: String,
    pub group_id: String,
    pub group_management_type: GroupManagementType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,
    pub group_type: GroupType,
    pub is_member: bool,
    pub is_owner: bool,
    pub same_team: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_external_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum GroupType {
    Team,
    UserManaged,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum GroupManagementType {
    UserManaged,
    CompanyManaged,
    SystemManaged,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteeInfoEmail {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum InviteeInfo {
    Email { email: String },
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum AccessLevel {
    Owner,
    Editor,
    Viewer,
    ViewerNoComment,
    Traverse,
    NoAccess,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMembershipInfo {
    pub access_type: AccessLevel,
    pub user: UserInfo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<MemberPermission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default)]
    pub is_inherited: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupMembershipInfo {
    pub access_type: AccessLevel,
    pub group: GroupInfo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<MemberPermission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default)]
    pub is_inherited: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteeMembershipInfo {
    pub access_type: AccessLevel,
    pub invitee: InviteeInfo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<MemberPermission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default)]
    pub is_inherited: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

// =============================================================================
// Folder / file metadata and policy
// =============================================================================

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum AclUpdatePolicy {
    Owner,
    Editors,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MemberPolicy {
    Team,
    Anyone,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ViewerInfoPolicy {
    Enabled,
    Disabled,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum AccessInheritance {
    Inherit,
    NoInherit,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderPolicy {
    pub acl_update_policy: AclUpdatePolicy,
    pub shared_link_policy: SharedLinkPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_policy: Option<MemberPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_member_policy: Option<MemberPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer_info_policy: Option<ViewerInfoPolicy>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedFolderMetadata {
    pub access_type: AccessLevel,
    pub is_inside_team_folder: bool,
    pub is_team_folder: bool,
    pub name: String,
    pub policy: FolderPolicy,
    pub preview_url: String,
    pub shared_folder_id: String,
    pub time_invited: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_display_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_team: Option<Team>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_shared_folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_display: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_lower: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_folder_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_inheritance: Option<AccessInheritance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<FolderPermission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_last_modified: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_metadata: Option<SharedContentLinkMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedFileMetadata {
    pub id: String,
    pub name: String,
    pub policy: FolderPolicy,
    pub preview_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_link_metadata: Option<SharedContentLinkMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_metadata: Option<SharedContentLinkMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_display_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_team: Option<Team>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_shared_folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_display: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_lower: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<FilePermission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_invited: Option<String>,
}

// =============================================================================
// Permission / action / reason taxonomy (was Vec<Value> before)
// =============================================================================

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharedLinkPolicy {
    Anyone,
    Members,
    Team,
    Other,
}

/// Used inside `LinkAudienceOption.disallowed_reason`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum LinkAudienceDisallowedReason {
    UserNotOnTeam,
    UserAccountType,
    PermissionDenied,
    Other,
}

/// Used inside member/folder/file permissions when an action is denied.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PermissionDeniedReason {
    UserNotSameTeamAsOwner,
    UserNotAllowedByOwner,
    TargetIsIndirectMember,
    TargetIsOwner,
    TargetIsSelf,
    TargetNotActive,
    FolderIsLimitedTeamFolder,
    OwnerNotOnTeam,
    PermissionDenied,
    RestrictedByTeam,
    UserAccountType,
    UserNotOnTeam,
    FolderIsInsideSharedFolder,
    RestrictedByParentFolder,
    InsufficientPlan(serde_json::Value),
    Other,
}

/// Member-level actions. Used by `update_file_member` and friends.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MemberAction {
    LeaveACopy,
    MakeEditor,
    MakeOwner,
    MakeViewer,
    MakeViewerNoComment,
    Remove,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberPermission {
    pub action: MemberAction,
    pub allow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<PermissionDeniedReason>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum FolderAction {
    ChangeOptions,
    DisableViewerInfo,
    EditContents,
    EnableViewerInfo,
    InviteEditor,
    InviteViewer,
    InviteViewerNoComment,
    RelinquishMembership,
    Unmount,
    Unshare,
    LeaveACopy,
    ShareLink,
    CreateLink,
    SetAccessInheritance,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderPermission {
    pub action: FolderAction,
    pub allow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<PermissionDeniedReason>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum FileAction {
    DisableViewerInfo,
    EditContents,
    EnableViewerInfo,
    InviteEditor,
    InviteViewer,
    InviteViewerNoComment,
    Unshare,
    RelinquishMembership,
    ShareLink,
    CreateLink,
    CreateViewLink,
    CreateEditLink,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilePermission {
    pub action: FileAction,
    pub allow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<PermissionDeniedReason>,
}

/// Shape of the link metadata embedded in `SharedFileMetadata`.
#[derive(Serialize, Deserialize, Debug)]
pub struct SharedContentLinkMetadata {
    pub audience_options: Vec<LinkAudience>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_audience: Option<LinkAudience>,
    pub link_permissions: Vec<LinkPermission>,
    pub password_protected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience_restricting_shared_folder: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkPermission {
    pub action: LinkAction,
    pub allow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<PermissionDeniedReason>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum LinkAction {
    ChangeAccessLevel,
    ChangeAudience,
    RemoveExpiry,
    RemovePassword,
    SetExpiry,
    SetPassword,
    Other,
}
