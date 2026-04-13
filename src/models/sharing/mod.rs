//! Types and endpoints for the Dropbox `sharing` namespace.
//!
//! **Skeleton — work in progress.** Dropbox declares 44 `sharing/*` endpoints
//! (see `src/endpoints/mod.rs`); this module currently wires up a subset
//! chosen for their minimal type surface. The rest are deliberate follow-up
//! work because complete typed coverage of shared-link metadata, folder
//! members, and member policies is a substantial per-struct audit on top of
//! the 44 endpoint files.
//!
//! Currently wired:
//! - `revoke_shared_link`
//! - `list_shared_links`
//!
//! Not yet wired (44 - 2 = 42): `add_file_member`, `add_folder_member`,
//! `check_job_status`, `check_remove_member_job_status`,
//! `check_share_job_status`, `create_shared_link_with_settings`,
//! `get_file_metadata`, `get_file_metadata_batch`, `get_folder_metadata`,
//! `get_shared_link_file`, `get_shared_link_metadata`, `list_file_members`,
//! `list_file_members_batch`, `list_file_members_continue`,
//! `list_folder_members`, `list_folder_members_continue`, `list_folders`,
//! `list_folders_continue`, `list_mountable_folders`,
//! `list_mountable_folders_continue`, `list_received_files`,
//! `list_received_files_continue`, `modify_shared_link_settings`,
//! `mount_folder`, `relinquish_file_membership`,
//! `relinquish_folder_membership`, `remove_file_member_2`,
//! `remove_folder_member`, `set_access_inheritance`, `share_folder`,
//! `transfer_folder`, `unmount_folder`, `unshare_file`, `unshare_folder`,
//! `update_file_member`, `update_folder_member`, `update_folder_policy`.
//!
//! Reference: <https://www.dropbox.com/developers/documentation/http/documentation#sharing>

pub mod list_shared_links;
pub mod revoke_shared_link;

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

/// Result of `list_shared_links`. The `links` field is typed as
/// `serde_json::Value` for now so we don't have to exhaustively model every
/// variant of `SharedLinkMetadata` (file vs folder) and its nested
/// permissions tree. Callers can still inspect fields generically.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListSharedLinksResult {
    pub links: Vec<serde_json::Value>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
