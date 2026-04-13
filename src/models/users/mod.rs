//! Types and endpoints for the Dropbox `users` namespace.
//!
//! Reference: <https://www.dropbox.com/developers/documentation/http/documentation#users>

pub mod features_get_values;
pub mod get_account;
pub mod get_account_batch;
pub mod get_current_account;
pub mod get_space_usage;

use serde::{Deserialize, Serialize};

// /users/get_current_account — FullAccount
#[derive(Serialize, Deserialize, Debug)]
pub struct FullAccount {
    pub account_id: String,
    pub name: Name,
    pub email: String,
    pub email_verified: bool,
    pub disabled: bool,
    pub locale: String,
    pub referral_link: String,
    pub is_paired: bool,
    pub account_type: AccountType,
    pub root_info: RootInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<FullTeam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
}

// /users/get_account — BasicAccount
#[derive(Serialize, Deserialize, Debug)]
pub struct BasicAccount {
    pub account_id: String,
    pub name: Name,
    pub email: String,
    pub email_verified: bool,
    pub disabled: bool,
    pub is_teammate: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub given_name: String,
    pub surname: String,
    pub familiar_name: String,
    pub display_name: String,
    pub abbreviated_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum AccountType {
    Basic,
    Pro,
    Business,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum RootInfo {
    User(UserRootInfo),
    Team(TeamRootInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRootInfo {
    pub root_namespace_id: String,
    pub home_namespace_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamRootInfo {
    pub root_namespace_id: String,
    pub home_namespace_id: String,
    pub home_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullTeam {
    pub id: String,
    pub name: String,
    pub sharing_policies: TeamSharingPolicies,
    pub office_addin_policy: OfficeAddInPolicy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamSharingPolicies {
    pub shared_folder_member_policy: SharedFolderMemberPolicy,
    pub shared_folder_join_policy: SharedFolderJoinPolicy,
    pub shared_link_create_policy: SharedLinkCreatePolicy,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharedFolderMemberPolicy {
    Team,
    Anyone,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharedFolderJoinPolicy {
    FromTeamOnly,
    FromAnyone,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharedLinkCreatePolicy {
    DefaultPublic,
    DefaultTeamOnly,
    TeamOnly,
    DefaultNoOne,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum OfficeAddInPolicy {
    Disabled,
    Enabled,
    Other,
}

// /users/get_account — arg
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccountArg {
    pub account_id: String,
}

// /users/get_account_batch — arg + result
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccountBatchArg {
    pub account_ids: Vec<String>,
}

// /users/get_space_usage — result
#[derive(Serialize, Deserialize, Debug)]
pub struct SpaceUsage {
    pub used: u64,
    pub allocation: SpaceAllocation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SpaceAllocation {
    Individual(IndividualSpaceAllocation),
    Team(TeamSpaceAllocation),
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualSpaceAllocation {
    pub allocated: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamSpaceAllocation {
    pub used: u64,
    pub allocated: u64,
    pub user_within_team_space_allocated: u64,
    pub user_within_team_space_limit_type: MemberSpaceLimitType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MemberSpaceLimitType {
    Off,
    AlertOnly,
    StopSync,
    Other,
}

// /users/features/get_values
#[derive(Serialize, Deserialize, Debug)]
pub struct UserFeaturesGetValuesBatchArg {
    pub features: Vec<UserFeature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFeaturesGetValuesBatchResult {
    pub values: Vec<UserFeatureValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UserFeature {
    PaperAsFiles,
    FileLocking,
    Other,
}

/// Dropbox wraps each user-feature value as `{".tag": "<feature>", "<feature>": {...}}`
/// where the inner object carries the actual value. Using `#[serde(tag = ".tag")]`
/// with struct variants matches that shape.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UserFeatureValue {
    PaperAsFiles { paper_as_files: PaperAsFilesValue },
    FileLocking { file_locking: FileLockingValue },
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PaperAsFilesValue {
    Enabled { enabled: bool },
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum FileLockingValue {
    Enabled { enabled: bool },
    Other,
}
