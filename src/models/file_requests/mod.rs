mod count;
mod create;
mod delete;
mod delete_all_closed;
mod get;
mod list;
mod list_continue;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CountFileRequestsResult {
    file_request_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFileRequestArgs {
    title: String,
    destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<FileRequestDeadline>,
    open: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_project_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFileRequestResult(FileRequest);

#[derive(Serialize, Deserialize, Debug)]
pub struct Deadline {
    deadline: DateTime<Utc>,
    allow_late_uploads: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRequestDeadline {
    deadline: DateTime<Utc>,
    allow_late_uploads: Option<GracePeriod>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum GracePeriod {
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
    ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFileRequestResult {
    file_requests: Vec<FileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAllClosedFileRequestsResult {
    file_requests: Vec<FileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedFileRequest {
    id: String,
    title: String,
    destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<FileRequestDeadline>,
    url: String,
    open: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileRequestArgs {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileRequestResult(FileRequest);

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsArgs {
    limit: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsResult {
    file_requests: Vec<FileRequest>,
    cursor: String,
    has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRequest {
    id: String,
    url: String,
    title: String,
    created: DateTime<Utc>,
    is_open: bool,
    file_count: i64,
    destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<FileRequestDeadline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_project_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsContinueArgs {
    cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsContinueResult {
    file_requests: Vec<FileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFileRequestArgs {
    id: String,
    title: Option<String>,
    destination: Option<String>,
    deadline: Option<Deadline>,
    open: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFileRequestResult {
    id: String,
    url: String,
    title: String,
    destination: String,
    deadline: Option<FileRequestDeadline>,
    open: bool,
}
