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

use std::collections::HashMap;

// Common Metadata Structs

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum Metadata {
    #[serde(rename = "file")]
    File(FileMetadata),
    #[serde(rename = "folder")]
    Folder(FolderMetadata),
    #[serde(rename = "deleted")]
    Deleted(DeletedMetadata),
}

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub id: String,
    pub client_modified: String,
    pub server_modified: String,
    pub rev: String,
    pub size: u64,
    pub path_lower: Option<String>,
    pub path_display: Option<String>,
    pub content_hash: Option<String>,
    pub property_groups: Option<Vec<PropertyGroup>>,
    pub file_lock_info: Option<FileLockMetadata>,
}

#[derive(Serialize, Deserialize)]
pub struct FolderMetadata {
    pub name: String,
    pub id: String,
    pub path_lower: Option<String>,
    pub path_display: Option<String>,
    pub property_groups: Option<Vec<PropertyGroup>>,
}

#[derive(Serialize, Deserialize)]
pub struct DeletedMetadata {
    pub name: String,
    pub path_lower: Option<String>,
    pub path_display: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PropertyGroup {
    pub template_id: String,
    pub fields: Vec<PropertyField>,
}

#[derive(Serialize, Deserialize)]
pub struct PropertyField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileLockMetadata {
    pub is_lockholder: Option<bool>,
    pub lockholder_name: Option<String>,
    pub created: Option<String>,
}

// Error Structs

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ApiError {
    #[serde(rename = "path_lookup")]
    PathLookup(LookupError),
    #[serde(rename = "path_write")]
    PathWrite(WriteError),
    #[serde(rename = "internal_error")]
    InternalError,
    #[serde(rename = "invalid_async_job_id")]
    InvalidAsyncJobId,
    #[serde(rename = "other")]
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum LookupError {
    #[serde(rename = "malformed_path")]
    MalformedPath(Option<String>),
    #[serde(rename = "not_found")]
    NotFound,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum WriteError {
    #[serde(rename = "malformed_path")]
    MalformedPath(Option<String>),
    #[serde(rename = "conflict")]
    Conflict(ConflictType),
    #[serde(rename = "insufficient_space")]
    InsufficientSpace,
    #[serde(rename = "disallowed_name")]
    DisallowedName,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ConflictType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file_ancestor")]
    FileAncestor,
}

// files/copy_v2

#[derive(Serialize, Deserialize)]
pub struct CopyArgs {
    pub from_path: String,
    pub to_path: String,
    pub allow_shared_folder: Option<bool>,
    pub autorename: Option<bool>,
    pub allow_ownership_transfer: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CopyResult {
    pub metadata: Metadata,
}

// files/copy_batch_v2

#[derive(Serialize, Deserialize)]
pub struct CopyBatchArgs {
    pub autorename: bool,
    pub entries: Vec<RelocationPath>,
}

#[derive(Serialize, Deserialize)]
pub struct RelocationPath {
    pub from_path: String,
    pub to_path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum RelocationBatchV2Launch {
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
}

#[derive(Serialize, Deserialize)]
pub struct RelocationBatchV2Result {
    pub entries: Vec<RelocationBatchResultEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum RelocationBatchResultEntry {
    #[serde(rename = "success")]
    Success { success: Metadata },
    #[serde(rename = "failure")]
    Failure { failure: RelocationBatchErrorEntry },
}

#[derive(Serialize, Deserialize)]
pub struct RelocationBatchErrorEntry {
    pub relocation_error: Option<RelocationError>,
}

// files/copy_batch/check_v2

#[derive(Serialize, Deserialize)]
pub struct PollArg {
    pub async_job_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum RelocationBatchV2JobStatus {
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/copy_reference/get

#[derive(Serialize, Deserialize)]
pub struct GetCopyReferenceArgs {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCopyReferenceResult {
    pub copy_reference: String,
    pub expires: String,
    pub metadata: Metadata,
}

// files/copy_reference/save

#[derive(Serialize, Deserialize)]
pub struct SaveCopyReferenceArgs {
    pub copy_reference: String,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaveCopyReferenceResult {
    pub metadata: Metadata,
}

// files/create_folder_v2

#[derive(Serialize, Deserialize)]
pub struct CreateFolderArgs {
    pub path: String,
    pub autorename: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateFolderResult {
    pub metadata: FolderMetadata,
}

// files/create_folder_batch

#[derive(Serialize, Deserialize)]
pub struct CreateFolderBatchArgs {
    pub paths: Vec<String>,
    pub autorename: Option<bool>,
    pub force_async: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum CreateFolderBatchLaunch {
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
    #[serde(rename = "complete")]
    Complete(CreateFolderBatchResult),
}

#[derive(Serialize, Deserialize)]
pub struct CreateFolderBatchResult {
    pub entries: Vec<CreateFolderBatchResultEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum CreateFolderBatchResultEntry {
    #[serde(rename = "success")]
    Success(CreateFolderEntryResult),
    #[serde(rename = "failure")]
    Failure(CreateFolderEntryError),
}

#[derive(Serialize, Deserialize)]
pub struct CreateFolderEntryResult {
    pub metadata: FolderMetadata,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum CreateFolderEntryError {
    #[serde(rename = "path")]
    Path(WriteError),
}

// files/create_folder_batch/check

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum CreateFolderBatchJobStatus {
    #[serde(rename = "complete")]
    Complete(CreateFolderBatchResult),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/delete_v2

#[derive(Serialize, Deserialize)]
pub struct DeleteArgs {
    pub path: String,
    pub parent_rev: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteResult {
    pub metadata: Metadata,
}

// files/delete_batch

#[derive(Serialize, Deserialize)]
pub struct DeleteBatchArgs {
    pub entries: Vec<DeleteArg>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum DeleteBatchLaunch {
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
    #[serde(rename = "complete")]
    Complete(DeleteBatchResult),
}

#[derive(Serialize, Deserialize)]
pub struct DeleteBatchResult {
    pub entries: Vec<DeleteBatchResultEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum DeleteBatchResultEntry {
    #[serde(rename = "success")]
    Success(DeleteBatchResultData),
    #[serde(rename = "failure")]
    Failure(DeleteError),
}

#[derive(Serialize, Deserialize)]
pub struct DeleteBatchResultData {
    pub metadata: Metadata,
}

// files/delete_batch/check

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum DeleteBatchJobStatus {
    #[serde(rename = "complete")]
    Complete(DeleteBatchResult),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/download

#[derive(Serialize, Deserialize)]
pub struct DownloadArg {
    pub path: String,
    pub rev: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FileMetadataV2 {
    pub client_modified: String,
    pub server_modified: String,
    pub rev: String,
    pub size: u64,
    pub path_lower: String,
    pub path_display: String,
    pub content_hash: String,
    pub property_groups: Option<Vec<PropertyGroup>>,
    pub file_lock_info: Option<FileLockMetadata>,
}

// files/download_zip

#[derive(Serialize, Deserialize)]
pub struct DownloadZipArg {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadZipResult {
    pub metadata: FolderMetadata,
}

// files/export

#[derive(Serialize, Deserialize)]
pub struct ExportArgs {
    pub path: String,
    pub export_format: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ExportResult {
    pub export_metadata: ExportMetadata,
    pub file_metadata: FileMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct ExportMetadata {
    pub name: String,
    pub size: u64,
    pub export_hash: Option<String>,
}

// files/get_file_lock_batch

#[derive(Serialize, Deserialize)]
pub struct LockFileBatchArgs {
    pub entries: Vec<LockFileArg>,
}

#[derive(Serialize, Deserialize)]
pub struct LockFileArg {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct LockFileBatchResult {
    pub entries: Vec<LockFileResultEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum LockFileResultEntry {
    #[serde(rename = "success")]
    Success(LockFileResult),
    #[serde(rename = "failure")]
    Failure(LockFileError),
}

#[derive(Serialize, Deserialize)]
pub struct LockFileResult {
    pub metadata: Metadata,
    pub lock: Option<FileLock>,
}

#[derive(Serialize, Deserialize)]
pub struct FileLock {
    pub created: String,
    pub lock_holder_account_id: String,
    pub lock_holder_team_id: Option<String>,
}

// files/get_metadata

#[derive(Serialize, Deserialize)]
pub struct GetMetadataArgs {
    pub path: String,
    pub include_media_info: Option<bool>,
    pub include_deleted: Option<bool>,
    pub include_has_explicit_shared_members: Option<bool>,
    pub include_property_groups: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetMetadataResult {
    pub metadata: Metadata,
}

// files/get_temporary_link

#[derive(Serialize, Deserialize)]
pub struct GetTemporaryLinkArgs {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetTemporaryLinkResult {
    pub link: String,
    pub metadata: Metadata,
}

// files/get_temporary_upload_link

#[derive(Serialize, Deserialize)]
pub struct GetTemporaryUploadLinkArgs {
    pub commit_info: CommitInfo,
    pub duration: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct CommitInfo {
    pub path: String,
    pub mode: WriteMode,
    pub autorename: Option<bool>,
    pub client_modified: Option<String>,
    pub mute: Option<bool>,
    pub property_groups: Option<Vec<PropertyGroup>>,
    pub strict_conflict: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum WriteMode {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "update")]
    Update { rev: String },
}

#[derive(Serialize, Deserialize)]
pub struct GetTemporaryUploadLinkResult {
    pub link: String,
}

// files/get_thumbnail_v2

#[derive(Serialize, Deserialize)]
pub struct ThumbnailV2Arg {
    pub resource: PathOrLink,
    pub format: ThumbnailFormat,
    pub size: ThumbnailSize,
    pub mode: ThumbnailMode,
    pub quality: ThumbnailQuality,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum PathOrLink {
    Path { path: String },
    Link(SharedLinkFileInfo),
}

#[derive(Serialize, Deserialize)]
pub struct SharedLinkFileInfo {
    pub url: String,
    pub path: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ThumbnailFormat {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ThumbnailSize {
    #[serde(rename = "w32h32")]
    W32h32,
    #[serde(rename = "w64h64")]
    W64h64,
    #[serde(rename = "w128h128")]
    W128h128,
    #[serde(rename = "w256h256")]
    W256h256,
    #[serde(rename = "w480h320")]
    W480h320,
    #[serde(rename = "w640h480")]
    W640h480,
    #[serde(rename = "w960h640")]
    W960h640,
    #[serde(rename = "w1024h768")]
    W1024h768,
    #[serde(rename = "w2048h1536")]
    W2048h1536,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ThumbnailMode {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "bestfit")]
    Bestfit,
    #[serde(rename = "fitone_bestfit")]
    FitOneBestfit,
    #[serde(rename = "original")]
    Original,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ThumbnailQuality {
    #[serde(rename = "quality_80")]
    Quality80,
    #[serde(rename = "quality_90")]
    Quality90,
}

#[derive(Serialize, Deserialize)]
pub struct PreviewResult {
    pub file_metadata: Option<FileMetadata>,
    pub link_metadata: Option<SharedLinkFileInfo>,
}

// files/list_folder

#[derive(Serialize, Deserialize)]
pub struct ListFolderArgs {
    pub path: String,
    pub recursive: Option<bool>,
    pub include_media_info: Option<bool>,
    pub include_deleted: Option<bool>,
    pub include_has_explicit_shared_members: Option<bool>,
    pub include_mounted_folders: Option<bool>,
    pub limit: Option<u32>,
    pub shared_link: Option<SharedLink>,
    pub include_property_groups: Option<Vec<String>>,
    pub include_non_downloadable_files: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SharedLink {
    pub url: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ListFolderResult {
    pub entries: Vec<Metadata>,
    pub cursor: String,
    pub has_more: bool,
}

// files/list_folder/continue

#[derive(Serialize, Deserialize)]
pub struct ListFolderContinueArgs {
    pub cursor: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListFolderContinueResult {
    pub entries: Vec<Metadata>,
    pub cursor: String,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ListFolderContinueError {
    #[serde(rename = "path")]
    Path(LookupError),
    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "other")]
    Other,
}

// files/list_folder/get_latest_cursor

#[derive(Serialize, Deserialize)]
pub struct GetLatestCursorArgs {
    pub path: String,
    pub recursive: Option<bool>,
    pub include_media_info: Option<bool>,
    pub include_deleted: Option<bool>,
    pub include_has_explicit_shared_members: Option<bool>,
    pub include_mounted_folders: Option<bool>,
    pub limit: Option<u32>,
    pub shared_link: Option<SharedLink>,
    pub include_property_groups: Option<Vec<String>>,
    pub include_non_downloadable_files: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct GetLatestCursorResult {
    pub cursor: String,
}

// files/list_folder/longpoll

#[derive(Serialize, Deserialize)]
pub struct ListFolderLongpollArgs {
    pub cursor: String,
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct ListFolderLongpollResult {
    pub changes: bool,
    pub backoff: Option<u64>,
}

// files/list_revisions

#[derive(Serialize, Deserialize)]
pub struct ListRevisionsArgs {
    pub path: String,
    pub mode: Option<ListRevisionsMode>,
    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum ListRevisionsMode {
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "id")]
    Id,
}

#[derive(Serialize, Deserialize)]
pub struct ListRevisionsResult {
    pub entries: Vec<FileMetadata>,
    pub is_deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LockFileError {
    pub lock_conflict: Option<FileLock>,
}

// files/move_v2

#[derive(Serialize, Deserialize)]
pub struct MoveArgs {
    pub from_path: String,
    pub to_path: String,
    pub allow_shared_folder: Option<bool>,
    pub autorename: Option<bool>,
    pub allow_ownership_transfer: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct MoveResult {
    pub metadata: Metadata,
}

// files/move_batch_v2

#[derive(Serialize, Deserialize)]
pub struct MoveBatchArgs {
    pub autorename: Option<bool>,
    pub entries: Vec<RelocationPath>,
    pub allow_ownership_transfer: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum MoveBatchResult {
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
}

// files/move_batch/check_v2

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum MoveBatchJobStatus {
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/paper/create

#[derive(Serialize, Deserialize)]
pub struct PaperCreateArgs {
    pub path: String,
    pub import_format: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaperCreateResult {
    pub file_id: String,
    pub paper_revision: i64,
    pub result_path: String,
    pub url: String,
}

// files/paper/update

#[derive(Serialize, Deserialize)]
pub struct PaperUpdateArgs {
    pub path: String,
    pub import_format: String,
    pub doc_update_policy: String,
    pub paper_revision: Option<i64>,
}

pub struct PaperUpdateResult {
    pub paper_revision: i64,
}

// files/permanently_delete

#[derive(Serialize, Deserialize)]
pub struct PermanentlyDeleteArgs {
    pub path: String,
    pub parent_rev: Option<String>,
}

// files/restore

#[derive(Serialize, Deserialize)]
pub struct RestoreArgs {
    pub path: String,
    pub rev: String,
}

#[derive(Serialize, Deserialize)]
pub struct RestoreResult {
    pub metadata: Metadata,
}

// Common Metadata Structs

// files/save_url

#[derive(Serialize, Deserialize)]
pub struct SaveUrlArg {
    pub path: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum SaveUrlResult {
    Complete(FileMetadata),
    AsyncJobId { async_job_id: String },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum SaveUrlError {
    Path(LookupError),
    DownloadFailed,
    InvalidUrl,
    NotFound,
}

// files/save_url/check_job_status

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum SaveUrlJobStatus {
    InProgress,
    Complete(FileMetadata),
    Failed(SaveUrlError),
}

// files/search_v2

#[derive(Serialize, Deserialize)]
pub struct SearchV2Arg {
    pub query: String,
    pub options: Option<SearchOptions>,
    pub match_field_options: Option<SearchMatchFieldOptions>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchOptions {
    pub file_status: Option<String>,
    pub filename_only: Option<bool>,
    pub max_results: Option<u32>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchMatchFieldOptions {
    pub include_highlights: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchV2Result {
    pub matches: Vec<SearchMatch>,
    pub has_more: bool,
    pub cursor: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum SearchMetadata {
    Metadata(SearchFileMatch),
}

#[derive(Serialize, Deserialize)]
pub struct SearchFileMatch {
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
pub struct SearchMatch {
    pub metadata: SearchMetadata,
}

// files/search/continue_v2

#[derive(Serialize, Deserialize)]
pub struct SearchV2ContinueArg {
    pub cursor: String,
}

// files/tags/add

#[derive(Serialize, Deserialize)]
pub struct AddTagArg {
    pub path: String,
    pub tag_text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum AddTagError {
    Path(LookupError),
    TooManyTags,
}

// files/tags/get

#[derive(Serialize, Deserialize)]
pub struct GetTagsArg {
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetTagsResult {
    pub paths_to_tags: Vec<PathToTags>,
}

#[derive(Serialize, Deserialize)]
pub struct PathToTags {
    pub path: String,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum Tag {
    UserGeneratedTag(UserGeneratedTag),
}

#[derive(Serialize, Deserialize)]
pub struct UserGeneratedTag {
    pub tag_text: String,
}

// files/tags/remove

#[derive(Serialize, Deserialize)]
pub struct RemoveTagArg {
    pub path: String,
    pub tag_text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum RemoveTagError {
    Path(LookupError),
    TagNotPresent,
}

// files/unlock_file_batch

#[derive(Serialize, Deserialize)]
pub struct UnlockFileBatchArg {
    pub entries: Vec<UnlockFileArg>,
}

#[derive(Serialize, Deserialize)]
pub struct UnlockFileArg {
    pub path: String,
}

// files/upload

#[derive(Serialize, Deserialize)]
pub struct UploadArg {
    pub path: String,
    pub mode: WriteMode,
    pub autorename: Option<bool>,
    pub client_modified: Option<String>,
    pub mute: Option<bool>,
    pub property_groups: Option<Vec<PropertyGroup>>,
    pub strict_conflict: Option<bool>,
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadError {
    Path(WriteError),
    PropertiesError,
    PayloadTooLarge,
    ContentHashMismatch,
}

// files/upload_session/append_v2

#[derive(Serialize, Deserialize)]
pub struct UploadSessionAppendArg {
    pub cursor: UploadSessionCursor,
    pub close: Option<bool>,
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadSessionCursor {
    pub session_id: String,
    pub offset: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadSessionAppendError {
    NotFound,
    IncorrectOffset,
    Closed,
    TooLarge,
    ConcurrentSessionInvalidOffset,
    ConcurrentSessionInvalidDataSize,
    PayloadTooLarge,
    ContentHashMismatch,
}

// files/upload_session/append_batch

#[derive(Serialize, Deserialize)]
pub struct UploadSessionAppendBatchArg {
    pub entries: Vec<UploadSessionAppendBatchEntry>,
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadSessionAppendBatchEntry {
    pub cursor: UploadSessionCursor,
    pub close: bool,
    pub length: u64,
}

#[derive(Serialize, Deserialize)]
pub struct UploadSessionAppendBatchResult {
    pub entries: Vec<UploadSessionAppendBatchResultEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadSessionAppendBatchResultEntry {
    Success,
    Failure(UploadSessionAppendError),
}

// files/upload_session/finish

#[derive(Serialize, Deserialize)]
pub struct UploadSessionFinishArg {
    pub cursor: UploadSessionCursor,
    pub commit: CommitInfo,
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadSessionFinishError {
    LookupFailed,
    Path(LookupError),
    PropertiesError,
    TooManySharedFolderTargets,
    TooManyWriteOperations,
    ConcurrentSessionDataNotAllowed,
    ConcurrentSessionNotClosed,
    ConcurrentSessionMissingData,
    PayloadTooLarge,
    ContentHashMismatch,
}

// files/upload_session/finish_batch_v2

#[derive(Serialize, Deserialize)]
pub struct UploadSessionFinishBatchArg {
    pub entries: Vec<UploadSessionFinishBatchEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadSessionFinishBatchEntry {
    pub cursor: UploadSessionCursor,
    pub commit: CommitInfo,
}

#[derive(Serialize, Deserialize)]
pub struct UploadSessionFinishBatchResult {
    pub entries: Vec<UploadSessionFinishBatchResultEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadSessionFinishBatchResultEntry {
    Success(FileMetadataV2),
    Failure(UploadSessionFinishError),
}

// files/upload_session/finish_batch/check

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadSessionFinishBatchJobStatus {
    InProgress,
    Complete(UploadSessionFinishBatchResult),
}

// files/upload_session/start

#[derive(Serialize, Deserialize)]
pub struct UploadSessionStartArg {
    pub close: Option<bool>,
    pub session_type: Option<UploadSessionType>,
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadSessionStartResult {
    pub session_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadSessionStartError {
    ConcurrentSessionDataNotAllowed,
    ConcurrentSessionCloseNotAllowed,
    PayloadTooLarge,
    ContentHashMismatch,
}

// files/upload_session/start_batch

#[derive(Serialize, Deserialize)]
pub struct UploadSessionStartBatchArg {
    pub num_sessions: u64,
    pub session_type: Option<UploadSessionType>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadSessionStartBatchResult {
    pub session_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum UploadSessionType {
    #[serde(rename = "sequential")]
    Sequential,
    #[serde(rename = "concurrent")]
    Concurrent,
}

// RelocationError

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum RelocationError {
    #[serde(rename = "from_lookup")]
    FromLookup(LookupError),
    #[serde(rename = "from_write")]
    FromWrite(WriteError),
    #[serde(rename = "to")]
    To(WriteError),
    #[serde(rename = "cant_copy_shared_folder")]
    CantCopySharedFolder,
    #[serde(rename = "cant_nest_shared_folder")]
    CantNestSharedFolder,
    #[serde(rename = "cant_move_folder_into_itself")]
    CantMoveFolderIntoItself,
    #[serde(rename = "too_many_files")]
    TooManyFiles,
    #[serde(rename = "duplicated_or_nested_paths")]
    DuplicatedOrNestedPaths,
    #[serde(rename = "cant_transfer_ownership")]
    CantTransferOwnership,
    #[serde(rename = "insufficient_quota")]
    InsufficientQuota,
    #[serde(rename = "internal_error")]
    InternalError,
    #[serde(rename = "cant_move_shared_folder")]
    CantMoveSharedFolder,
    #[serde(rename = "cant_move_into_vault")]
    CantMoveIntoVault(MoveIntoVaultError),
    #[serde(rename = "cant_move_into_family")]
    CantMoveIntoFamily(MoveIntoFamilyError),
    #[serde(rename = "other")]
    Other,
}

// MoveIntoVaultError

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum MoveIntoVaultError {
    #[serde(rename = "is_shared_folder")]
    IsSharedFolder,
    #[serde(rename = "other")]
    Other,
}

// MoveIntoFamilyError

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum MoveIntoFamilyError {
    #[serde(rename = "is_shared_folder")]
    IsSharedFolder,
    #[serde(rename = "other")]
    Other,
}

// DeleteArg

#[derive(Serialize, Deserialize)]
pub struct DeleteArg {
    pub path: String,
    pub parent_rev: Option<String>,
}

// DeleteError

#[derive(Serialize, Deserialize)]
#[serde(tag = ".tag")]
pub enum DeleteError {
    #[serde(rename = "path_lookup")]
    PathLookup(LookupError),
    #[serde(rename = "path_write")]
    PathWrite(WriteError),
    #[serde(rename = "too_many_write_operations")]
    TooManyWriteOperations,
    #[serde(rename = "too_many_files")]
    TooManyFiles,
    #[serde(rename = "other")]
    Other,
}
