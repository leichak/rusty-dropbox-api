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
    pub links: Vec<serde_json::Value>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
