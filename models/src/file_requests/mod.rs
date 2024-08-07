mod count;
mod create;
mod delete;
mod delete_all_closed;
mod get;
mod list;
mod list_continue;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CountFileRequestsResult {
    open_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFileRequestArgs {
    title: String,
    destination: String,
    deadline: Option<Deadline>,
    open: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFileRequestResult {
    id: String,
    url: String,
    title: String,
    destination: String,
    deadline: Option<DeadlineResult>,
    open: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deadline {
    deadline: String,
    allow_late_uploads: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeadlineResult {
    deadline: String,
    allow_late_uploads: Option<bool>,
    is_expired: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFileRequestArgs {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFileRequestResult {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAllClosedFileRequestsResult {
    file_requests: Vec<DeletedFileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedFileRequest {
    id: String,
    title: String,
    destination: String,
    deadline: Option<DeadlineResult>,
    url: String,
    open: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileRequestArgs {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFileRequestResult {
    id: String,
    url: String,
    title: String,
    destination: String,
    deadline: Option<DeadlineResult>,
    open: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsArgs {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFileRequestsResult {
    file_requests: Vec<FileRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRequest {
    id: String,
    title: String,
    destination: String,
    deadline: Option<DeadlineResult>,
    url: String,
    open: bool,
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
    deadline: Option<DeadlineResult>,
    open: bool,
}
