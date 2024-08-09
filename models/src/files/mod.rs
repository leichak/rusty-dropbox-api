mod copy;
mod copy_batch;
mod copy_batch_check;
mod copy_reference_get;
mod copy_reference_save;
mod create_folder;
mod create_folder_batch;
mod create_folder_batch_check;
mod delete;
mod delete_batch;
mod delete_batch_check;
mod download;
mod download_zip;
mod export;
mod get_file_lock_batch;
mod get_metadata;
mod get_preview;
mod get_temporary_link;
mod get_temporary_upload_link;
mod get_thumbnail;
mod get_thumbnail_batch;
mod list_folder;
mod list_folder_get_latest_cursor;
mod list_folder_longpoll;
mod list_folders_continue;
mod list_revisions;
mod lock_file_batch;
mod r#move;
mod move_batch;
mod move_batch_check;
mod paper_create;
mod paper_update;
mod permanently_delete;
mod restore;
mod save_url;
mod save_url_check_job_status;
mod search;
mod search_continue;
mod tags_add;
mod tags_get;
mod tags_remove;
mod unlock_file_batch;
mod upload;
mod upload_session_append;
mod upload_session_append_batch;
mod upload_session_finish;
mod upload_session_finish_batch;
mod upload_session_finish_batch_check;
mod upload_session_start;
mod upload_session_start_batch;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyFileArgs {
    pub allow_ownership_transfer: Option<bool>,
    pub allow_shared_folder: Option<bool>,
    pub autorename: Option<bool>,
    pub from_path: String,
    pub to_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyFileResult {
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum Metadata {
    File(FileMetadata),
    Folder(FolderMetadata),
    Deleted(DeletedMetadata),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMetadata {
    pub client_modified: Option<String>,
    pub content_hash: Option<String>,
    pub file_lock_info: Option<FileLockMetadata>,
    pub has_explicit_shared_members: Option<bool>,
    pub id: String,
    pub is_downloadable: Option<bool>,
    pub name: String,
    pub path_display: Option<String>,
    pub path_lower: Option<String>,
    pub property_groups: Option<Vec<PropertyGroup>>,
    pub rev: String,
    pub server_modified: String,
    pub sharing_info: Option<FileSharingInfo>,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileLockMetadata {
    pub created: String,
    pub is_lockholder: bool,
    pub lockholder_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyGroup {
    pub fields: Vec<PropertyField>,
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSharingInfo {
    pub modified_by: String,
    pub parent_shared_folder_id: Option<String>,
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderMetadata {
    pub name: String,
    pub id: String,
    pub path_lower: Option<String>,
    pub path_display: Option<String>,
    pub parent_shared_folder_id: Option<String>,
    pub preview_url: Option<String>,
    pub sharing_info: Option<FolderSharingInfo>,
    pub property_groups: Option<Vec<PropertyGroup>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderSharingInfo {
    pub shared_folder_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedMetadata {
    pub name: String,
    pub path_lower: Option<String>,
    pub path_display: Option<String>,
    pub parent_shared_folder_id: Option<String>,
    pub preview_url: Option<String>,
}

// /copy_batch endpoint
#[derive(Serialize, Deserialize, Debug)]
pub struct CopyBatchArgs {
    pub autorename: Option<bool>,
    pub entries: Vec<RelocationPath>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelocationPath {
    pub from_path: String,
    pub to_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum CopyBatchResult {
    Complete(RelocationBatchResult),
    AsyncJobId { async_job_id: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelocationBatchResult {
    pub entries: Vec<RelocationBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum RelocationBatchResultEntry {
    Success(Metadata),
    Failure(RelocationBatchErrorEntry),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RelocationBatchErrorEntry {
    RelocationError(RelocationError),
    InternalError,
    TooManyWriteOperations,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum RelocationError {
    CantCopySharedFolder,
    CantNestSharedFolder,
    CantMoveFolderIntoItself,
    TooManyFiles,
    DuplicatedOrNestedPaths,
    CantTransferOwnership,
    InsufficientQuota,
    CantMoveSharedFolder,
    CantMoveIntoVault,
    CantMoveIntoFamily,
}

// /copy_batch/check endpoint
#[derive(Serialize, Deserialize, Debug)]
pub struct CopyBatchCheckArgs {
    pub async_job_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum CopyBatchCheckResult {
    InProgress,
    Complete(RelocationBatchResult),
}

// SaveCopyReferenceArg struct
#[derive(Serialize, Deserialize, Debug)]
pub struct SaveCopyReferenceArg {
    pub copy_reference: String,
    pub path: String,
}

// Struct for Get Copy Reference Arguments
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCopyReferenceArg {
    pub path: String,
}

// GetCopyReferenceResult struct
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCopyReferenceResult {
    pub copy_reference: String,
    pub expires: String,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveCopyReferenceResult {
    pub metadata: Metadata,
}
// CreateFolderArg struct
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderArg {
    pub autorename: bool,
    pub path: String,
}

// CreateFolderResult struct
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderResult {
    pub metadata: FolderMetadata,
}

// Create Folder Batch Arguments
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderBatchArg {
    pub paths: Vec<String>,
    pub autorename: Option<bool>,
    pub force_async: Option<bool>,
}

// Create Folder Batch Result
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum CreateFolderBatchLaunch {
    AsyncJobId { async_job_id: String },
    Complete(CreateFolderBatchResult),
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderBatchResult {
    pub entries: Vec<CreateFolderBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum CreateFolderBatchResultEntry {
    Success(CreateFolderEntryResult),
    Failure(CreateFolderEntryError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderEntryResult {
    pub metadata: FolderMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum CreateFolderEntryError {
    PathWriteError,
}

// Create Folder Batch Check Arguments
#[derive(Serialize, Deserialize, Debug)]
pub struct PollArg {
    pub async_job_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum CreateFolderBatchJobStatus {
    InProgress,
    Complete(CreateFolderBatchResult),
    Failed(CreateFolderBatchError),
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderBatchError {
    // Define specific error fields as needed
}

// Delete Arguments
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteArg {
    pub path: String,
    pub parent_rev: Option<String>,
}

// Delete Result
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResult {
    pub metadata: Metadata,
}

// Delete Batch Arguments
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteBatchArg {
    pub entries: Vec<DeleteArg>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum DeleteBatchLaunch {
    AsyncJobId { async_job_id: String },
    Complete(DeleteBatchResult),
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteBatchResult {
    pub entries: Vec<DeleteBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum DeleteBatchResultEntry {
    Success(DeleteBatchResultData),
    Failure(DeleteError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteBatchResultData {
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag")]
pub enum DeleteError {
    PathLookupError,
    PathWriteError,
    TooManyWriteOperations,
    TooManyFiles,
    Other,
}

/// Struct representing the arguments to download a file.
#[derive(Serialize, Deserialize, Debug)]
struct DownloadArgs {
    /// Path to the file to download, relative to the user's root folder.
    path: String,
    /// Optional field representing the unique identifier for the file revision.
    rev: Option<String>,
}

/// Struct representing the result of downloading a file.
#[derive(Serialize, Deserialize, Debug)]
struct DownloadResult {
    /// Metadata for the downloaded file.
    metadata: FileMetadata,
    /// Binary content of the downloaded file.
    content: Vec<u8>,
}

/// Struct representing the arguments to download a folder as a zip file.
#[derive(Serialize, Deserialize, Debug)]
struct DownloadZipArgs {
    /// Path to the folder to download, relative to the user's root folder.
    path: String,
}

/// Struct representing the result of downloading a folder as a zip file.
#[derive(Serialize, Deserialize, Debug)]
struct DownloadZipResult {
    /// The raw zip file data.
    content: Vec<u8>,
    /// Metadata for the downloaded folder.
    metadata: Option<FolderMetadata>, // Folder metadata might be optional
}

/// Struct representing sharing information for a file.
#[derive(Serialize, Deserialize, Debug)]
struct SharingInfo {
    shared_folder_id: Option<String>,
    shared_folder_name: Option<String>,
    permissions: Option<Permissions>,
}

/// Struct representing permissions related to the file.
#[derive(Serialize, Deserialize, Debug)]
struct Permissions {
    can_edit: bool,
    can_view: bool,
    can_comment: bool,
}
