pub mod count;
pub mod create;
pub mod delete;
pub mod delete_all_closed;
pub mod get;
pub mod list;
pub mod list_continue;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CountFileRequestsResult {
    pub file_request_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFileRequestArgs {
    pub title: String,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<FileRequestDeadline>,
    pub open: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_project_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFileRequestResult(FileRequest);

#[derive(Serialize, Deserialize, Debug)]
pub struct Deadline {
    pub deadline: DateTime<Utc>,
    pub allow_late_uploads: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRequestDeadline {
    pub deadline: DateTime<Utc>,
    pub allow_late_uploads: Option<GracePeriod>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GracePeriod {
    GracePeriodTagged(GracePeriodTagged),
    GracePeriodUntagged(GracePeriodUntagged),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = ".tag")]
enum GracePeriodTagged {
    OneDay,
    TwoDays,
    SevenDays,
    ThirtyDays,
    Always,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum GracePeriodUntagged {
    OneDay,
    TwoDays,
    SevenDays,
    ThirtyDays,
    Always,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFileRequestArgs {
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFileRequestResult {
    pub file_requests: Vec<FileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAllClosedFileRequestsResult {
    pub file_requests: Vec<FileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedFileRequest {
    pub id: String,
    pub title: String,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<FileRequestDeadline>,
    pub url: String,
    pub open: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileRequestArgs {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileRequestResult(FileRequest);

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsArgs {
    pub limit: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsResult {
    pub file_requests: Vec<FileRequest>,
    pub cursor: String,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRequest {
    pub id: String,
    pub url: String,
    pub title: String,
    pub created: DateTime<Utc>,
    pub is_open: bool,
    pub file_count: i64,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<FileRequestDeadline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_project_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsContinueArgs {
    pub cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsContinueResult {
    pub file_requests: Vec<FileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFileRequestArgs {
    pub id: String,
    pub title: Option<String>,
    pub destination: Option<String>,
    pub deadline: Option<Deadline>,
    pub open: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFileRequestResult {
    pub id: String,
    pub url: String,
    pub title: String,
    pub destination: String,
    pub deadline: Option<FileRequestDeadline>,
    pub open: bool,
}
