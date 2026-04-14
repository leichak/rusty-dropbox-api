// Wire types follow the Dropbox Stone IDL exactly. Stone tagged unions
// nest large structs as variants (e.g. `Metadata::File(FileMetadata)`),
// which makes `clippy::large_enum_variant` fire across this module. Boxing
// would change the public API, so we silence the lint at the module level.
#![allow(clippy::large_enum_variant)]

pub mod copy;
pub mod copy_batch;
pub mod copy_batch_check;
pub mod copy_reference_get;
pub mod copy_reference_save;
pub mod create_folder;
pub mod create_folder_batch;
pub mod create_folder_batch_check;
pub mod delete;
pub mod delete_batch;
pub mod delete_batch_check;
pub mod download;
pub mod download_zip;
pub mod export;
pub mod get_file_lock_batch;
pub mod get_metadata;
pub mod get_preview;
pub mod get_temporary_link;
pub mod get_temporary_upload_link;
pub mod get_thumbnail;
pub mod get_thumbnail_batch;
pub mod list_folder;
pub mod list_folder_get_latest_cursor;
pub mod list_folder_longpoll;
pub mod list_folders_continue;
pub mod list_revisions;
pub mod lock_file_batch;
pub mod r#move;
pub mod move_batch;
pub mod move_batch_check;
pub mod paper_create;
pub mod paper_update;
pub mod permanently_delete;
pub mod restore;
pub mod save_url;
pub mod save_url_check_job_status;
pub mod search;
pub mod search_continue;
pub mod tags_add;
pub mod tags_get;
pub mod tags_remove;
pub mod unlock_file_batch;
pub mod upload;
pub mod upload_session_append;
pub mod upload_session_append_batch;
pub mod upload_session_finish;
pub mod upload_session_finish_batch;
pub mod upload_session_finish_batch_check;
pub mod upload_session_start;
pub mod upload_session_start_batch;

use serde::{Deserialize, Serialize};

// Common Metadata Structs

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum Metadata {
    #[serde(rename = "file")]
    File(FileMetadata),
    #[serde(rename = "folder")]
    Folder(FolderMetadata),
    #[serde(rename = "deleted")]
    Deleted(DeletedMetadata),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataV2 {
    metadata: Metadata,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct FileMetadata {
//     pub name: String,
//     pub id: String,
//     pub client_modified: String,
//     pub server_modified: String,
//     pub rev: String,
//     pub size: u64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub path_lower: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub path_display: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub parent_shared_folder_id: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub preview_url: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub media_info: Option<MediaInfo>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub content_hash: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub property_groups: Option<Vec<PropertyGroup>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub file_lock_info: Option<FileLockMetadata>,
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct MediaInfo {

// }

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub id: String,
    pub client_modified: String,
    pub server_modified: String,
    pub rev: String,
    pub size: u64,
    pub path_lower: Option<String>,
    pub path_display: Option<String>,
    #[serde(
        rename = "parent_shared_folder_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_shared_folder_id: Option<String>,
    #[serde(rename = "preview_url", skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    #[serde(rename = "media_info", skip_serializing_if = "Option::is_none")]
    pub media_info: Option<MediaInfo>,
    #[serde(rename = "symlink_info", skip_serializing_if = "Option::is_none")]
    pub symlink_info: Option<SymlinkInfo>,
    #[serde(rename = "sharing_info", skip_serializing_if = "Option::is_none")]
    pub sharing_info: Option<FileSharingInfo>,
    #[serde(rename = "is_downloadable")]
    pub is_downloadable: bool,
    #[serde(rename = "export_info", skip_serializing_if = "Option::is_none")]
    pub export_info: Option<ExportInfo>,
    #[serde(rename = "property_groups", skip_serializing_if = "Option::is_none")]
    pub property_groups: Option<Vec<PropertyGroup>>,
    #[serde(
        rename = "has_explicit_shared_members",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_explicit_shared_members: Option<bool>,
    #[serde(rename = "content_hash", skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(rename = "file_lock_info", skip_serializing_if = "Option::is_none")]
    pub file_lock_info: Option<FileLockMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MediaInfo {
    Pending,
    Metadata(MediaMetadata),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub photo: Option<PhotoMetadata>,
    pub video: Option<VideoMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoMetadata {
    pub dimensions: Option<Dimensions>,
    pub location: Option<GpsCoordinates>,
    #[serde(rename = "time_taken", skip_serializing_if = "Option::is_none")]
    pub time_taken: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub dimensions: Option<Dimensions>,
    pub location: Option<GpsCoordinates>,
    #[serde(rename = "time_taken", skip_serializing_if = "Option::is_none")]
    pub time_taken: Option<String>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimensions {
    pub height: u64,
    pub width: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymlinkInfo {
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSharingInfo {
    pub read_only: bool,
    pub parent_shared_folder_id: String,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportInfo {
    #[serde(rename = "export_as", skip_serializing_if = "Option::is_none")]
    pub export_as: Option<String>,
    #[serde(rename = "export_options", skip_serializing_if = "Option::is_none")]
    pub export_options: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyGroup {
    pub template_id: String,
    pub fields: Vec<PropertyField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileLockMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_lockholder: Option<bool>,
    #[serde(rename = "lockholder_name", skip_serializing_if = "Option::is_none")]
    pub lockholder_name: Option<String>,
    #[serde(
        rename = "lockholder_account_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub lockholder_account_id: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderMetadata {
    pub name: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_lower: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_shared_folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_info: Option<FolderSharingInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_groups: Option<Vec<PropertyGroup>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderSharingInfo {
    pub read_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_shared_folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traverse_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedMetadata {
    pub name: String,
    pub path_lower: Option<String>,
    pub path_display: Option<String>,
}

// Error Structs

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum LookupError {
    #[serde(rename = "malformed_path")]
    MalformedPath(Option<String>),
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "not_file")]
    NotFile,
    #[serde(rename = "not_folder")]
    NotFolder,
    #[serde(rename = "restricted_content")]
    RestrictedContent,
    #[serde(rename = "unsupported_content_type")]
    UnsupportedContentType,
    #[serde(rename = "locked")]
    Locked,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum WriteError {
    #[serde(rename = "malformed_path")]
    MalformedPath(Option<String>),
    #[serde(rename = "conflict")]
    Conflict(ConflictType),
    #[serde(rename = "no_write_permission")]
    NoWritePermission,
    #[serde(rename = "insufficient_space")]
    InsufficientSpace,
    #[serde(rename = "disallowed_name")]
    DisallowedName,
    #[serde(rename = "team_folder")]
    TeamFolder,
    #[serde(rename = "operation_suppressed")]
    OperationSuppressed,
    #[serde(rename = "too_many_write_operations")]
    TooManyWriteOperations,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ConflictType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file_ancestor")]
    FileAncestor,
}

// files/copy_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyArgs {
    pub from_path: String,
    pub to_path: String,
    pub allow_shared_folder: Option<bool>,
    pub autorename: Option<bool>,
    pub allow_ownership_transfer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyResult {
    pub metadata: Metadata,
}

// files/copy_batch_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyBatchArgs {
    pub autorename: bool,
    pub entries: Vec<RelocationPath>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelocationPath {
    pub from_path: String,
    pub to_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum RelocationBatchV2Launch {
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelocationBatchV2Result {
    pub entries: Vec<RelocationBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum RelocationBatchResultEntry {
    #[serde(rename = "success")]
    Success { success: Metadata },
    #[serde(rename = "failure")]
    Failure { failure: RelocationBatchErrorEntry },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelocationBatchErrorEntry {
    pub relocation_error: Option<RelocationError>,
}

// files/copy_batch/check_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct AsyncJobCheckArgs {
    pub async_job_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum CopyBatchCheckResult {
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/copy_reference/get

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCopyReferenceArgs {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCopyReferenceResult {
    pub copy_reference: String,
    pub expires: String,
    pub metadata: Metadata,
}

// files/copy_reference/save

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveCopyReferenceArgs {
    pub copy_reference: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveCopyReferenceResult {
    pub metadata: Metadata,
}

// files/create_folder_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorename: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderResult {
    pub metadata: FolderMetadata,
}

// files/create_folder_batch

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderBatchArgs {
    pub paths: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorename: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_async: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum CreateFolderBatchLaunch {
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
    #[serde(rename = "complete")]
    Complete(CreateFolderBatchResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderBatchResult {
    pub entries: Vec<CreateFolderBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum CreateFolderBatchResultEntry {
    #[serde(rename = "success")]
    Success(CreateFolderEntryResult),
    #[serde(rename = "failure")]
    Failure(CreateFolderEntryError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderEntryResult {
    pub metadata: FolderMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum CreateFolderEntryError {
    #[serde(rename = "path")]
    Path(WriteError),
}

// files/create_folder_batch/check

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum CreateFolderBatchCheckResult {
    #[serde(rename = "complete")]
    Complete(CreateFolderBatchResult),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/delete_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_rev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResult {
    pub metadata: Metadata,
}

// files/delete_batch

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteBatchArgs {
    pub entries: Vec<DeleteArg>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum DeleteBatchLaunch {
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
    #[serde(rename = "complete")]
    Complete(DeleteBatchResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteBatchResult {
    pub entries: Vec<DeleteBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum DeleteBatchResultEntry {
    #[serde(rename = "success")]
    Success(DeleteBatchResultData),
    #[serde(rename = "failure")]
    Failure(DeleteError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteBatchResultData {
    pub metadata: Metadata,
}

// files/delete_batch/check

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum DeleteBatchJobStatus {
    #[serde(rename = "complete")]
    Complete(DeleteBatchResult),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/download

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadArg {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMetadataV2 {
    pub client_modified: String,
    pub server_modified: String,
    pub rev: String,
    pub size: u64,
    pub path_lower: String,
    pub path_display: String,
    pub content_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_groups: Option<Vec<PropertyGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_lock_info: Option<FileLockMetadata>,
}

// files/download_zip

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadZipArg {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadZipResult {
    pub metadata: FolderMetadata,
}

// files/export

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportResult {
    pub export_metadata: ExportMetadata,
    pub file_metadata: FileMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportMetadata {
    pub name: String,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_hash: Option<String>,
}

// files/get_preview

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPreviewArg {
    pub path: String,
}

// files/get_file_lock_batch

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFileBatchArgs {
    pub entries: Vec<LockFileArg>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFileArg {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFileBatchResult {
    pub entries: Vec<LockFileResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum LockFileResultEntry {
    #[serde(rename = "success")]
    Success(LockFileResult),
    #[serde(rename = "failure")]
    Failure(LockFileError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFileResult {
    pub metadata: Metadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<FileLock>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileLock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_holder_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_holder_team_id: Option<String>,
}

// files/get_metadata

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMetadataArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_media_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_has_explicit_shared_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_property_groups: Option<TemplateFilterBase>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMetadataResult(Metadata);

/// Filter for `include_property_groups` on list_folder / get_metadata /
/// search. Dropbox shape:
///   {".tag": "filter_some", "filter_some": ["ptid:..."]}
///   {".tag": "filter_none"}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum TemplateFilterBase {
    FilterSome { filter_some: Vec<String> },
    FilterNone,
}

// files/get_temporary_link

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTemporaryLinkArgs {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTemporaryLinkResult {
    pub link: String,
    pub metadata: FileMetadata,
}

// files/get_temporary_upload_link

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTemporaryUploadLinkArgs {
    pub commit_info: CommitInfo,
    pub duration: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitInfo {
    pub path: String,
    pub mode: WriteMode,
    pub autorename: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_modified: Option<String>,
    pub mute: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_groups: Option<Vec<PropertyGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_conflict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum WriteMode {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "update")]
    Update { rev: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTemporaryUploadLinkResult {
    pub link: String,
}

// files/get_thumbnail_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailArgs {
    entries: Vec<ThumbnailArg>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailArg {
    pub path: String,
    pub format: ThumbnailFormat,
    pub size: ThumbnailSize,
    pub mode: ThumbnailMode,
    pub quality: ThumbnailQuality,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailV2Arg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<PathOrLink>,
    pub format: ThumbnailFormat,
    pub size: ThumbnailSize,
    pub mode: ThumbnailMode,
    pub quality: ThumbnailQuality,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PathOrLink {
    #[serde(rename = "path")]
    Path { path: String },
    #[serde(rename = "link")]
    Link(SharedLinkFileInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedLinkFileInfo {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailEntry {
    metadata: FileMetadata,
    thumbnail: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum GetThumbnailBatchEntry {
    #[serde(rename = "success")]
    Success(ThumbnailEntry),
    #[serde(rename = "failure")]
    Failure(ThumbnailError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetThumbnailBatchResult {
    entries: Vec<GetThumbnailBatchEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetThumbnailResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_metadata: Option<FileMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_metadata: Option<MinimalFileLinkMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinimalFileLinkMetadata {
    url: String,
    rev: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "error_type", rename_all = "snake_case")]
pub enum ThumbnailError {
    PathLookupError(LookupError),
    UnsupportedExtension,
    UnsupportedImage,
    EncryptedContent,
    ConversionError,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ThumbnailFormat {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ThumbnailQuality {
    #[serde(rename = "quality_80")]
    Quality80,
    #[serde(rename = "quality_90")]
    Quality90,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPreviewResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_metadata: Option<FileMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_metadata: Option<SharedLinkFileInfo>,
}

// files/list_folder
#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_media_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_has_explicit_shared_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_mounted_folders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_link: Option<SharedLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_property_groups: Option<TemplateFilterBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_non_downloadable_files: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedLink {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderResult {
    pub entries: Vec<Metadata>,
    pub cursor: String,
    pub has_more: bool,
}

// files/list_folder/continue

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderContinueArgs {
    pub cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderContinueResult {
    pub entries: Vec<Metadata>,
    pub cursor: String,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ListFolderContinueError {
    #[serde(rename = "path")]
    Path(LookupError),
    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "other")]
    Other,
}

// files/list_folder/get_latest_cursor

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestCursorArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_media_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_has_explicit_shared_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_mounted_folders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_link: Option<SharedLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_property_groups: Option<TemplateFilterBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_non_downloadable_files: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestCursorResult {
    pub cursor: String,
}

// files/list_folder/longpoll

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderLongpollArgs {
    pub cursor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolderLongpollResult {
    pub changes: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff: Option<u64>,
}

// files/list_revisions

#[derive(Serialize, Deserialize, Debug)]
pub struct ListRevisionsArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ListRevisionsMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ListRevisionsMode {
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "id")]
    Id,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListRevisionsResult {
    pub entries: Vec<FileMetadata>,
    pub is_deleted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFileError {
    pub lock_conflict: Option<FileLock>,
}

// files/move_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveArgs {
    pub from_path: String,
    pub to_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_shared_folder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorename: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ownership_transfer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveResult {
    pub metadata: Metadata,
}

// files/move_batch_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveBatchArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorename: Option<bool>,
    pub entries: Vec<RelocationPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ownership_transfer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MoveBatchResult {
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
}

// files/move_batch/check_v2

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MoveBatchJobStatus {
    #[serde(rename = "complete")]
    Complete(RelocationBatchV2Result),
    #[serde(rename = "in_progress")]
    InProgress,
}

// files/paper/create

#[derive(Serialize, Deserialize, Debug)]
pub struct PaperCreateArgs {
    pub path: String,
    pub import_format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaperCreateResult {
    pub file_id: String,
    pub paper_revision: i64,
    pub result_path: String,
    pub url: String,
}

// files/paper/update

#[derive(Serialize, Deserialize, Debug)]
pub struct PaperUpdateArgs {
    pub path: String,
    pub import_format: String,
    pub doc_update_policy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_revision: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaperUpdateResult {
    pub paper_revision: i64,
}

// files/permanently_delete

#[derive(Serialize, Deserialize, Debug)]
pub struct PermanentlyDeleteArgs {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_rev: Option<String>,
}

// files/restore

#[derive(Serialize, Deserialize, Debug)]
pub struct RestoreArgs {
    pub path: String,
    pub rev: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestoreResult {
    pub metadata: FileMetadata,
}

// Common Metadata Structs

// files/save_url

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveUrlArg {
    pub path: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SaveUrlResult {
    #[serde(rename = "complete")]
    Complete(FileMetadata),
    #[serde(rename = "async_job_id")]
    AsyncJobId { async_job_id: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SaveUrlError {
    Path(LookupError),
    DownloadFailed,
    InvalidUrl,
    NotFound,
}

// files/save_url/check_job_status

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SaveUrlJobStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "complete")]
    Complete(FileMetadata),
    #[serde(rename = "failed")]
    Failed(SaveUrlError),
}

// files/search_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchV2Arg {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SearchOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_field_options: Option<SearchMatchFieldOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchMatchFieldOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_highlights: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchV2Result {
    pub matches: Vec<SearchMatchV2>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMatchV2 {
    pub metadata: MetadataV2,
    #[serde(rename = "match_type", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<SearchMatchTypeV2>,
    #[serde(rename = "highlight_spans", skip_serializing_if = "Option::is_none")]
    pub highlight_spans: Option<Vec<HighlightSpan>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SearchMatchTypeV2 {
    Filename,
    FileContent,
    FilenameAndContent,
    ImageContent,
    Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighlightSpan {
    #[serde(rename = "highlight_str")]
    pub highlight_str: String,
    #[serde(rename = "is_highlighted")]
    pub is_highlighted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SearchMetadata {
    Metadata(SearchFileMatch),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchFileMatch {
    pub metadata: Metadata,
}

// files/search/continue_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchV2ContinueArg {
    pub cursor: String,
}

// files/tags/add

#[derive(Serialize, Deserialize, Debug)]
pub struct AddTagArg {
    pub path: String,
    pub tag_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum AddTagError {
    Path(LookupError),
    TooManyTags,
}

// files/tags/get

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTagsArg {
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTagsResult {
    pub paths_to_tags: Vec<PathToTags>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathToTags {
    pub path: String,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum Tag {
    #[serde(rename = "user_generated_tag")]
    UserGeneratedTag(UserGeneratedTag),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGeneratedTag {
    pub tag_text: String,
}

// files/tags/remove

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveTagArg {
    pub path: String,
    pub tag_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum RemoveTagError {
    Path(LookupError),
    TagNotPresent,
}

// files/unlock_file_batch

#[derive(Serialize, Deserialize, Debug)]
pub struct UnlockFileBatchArg {
    pub entries: Vec<UnlockFileArg>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnlockFileArg {
    pub path: String,
}

// files/upload

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadArg {
    pub path: String,
    pub mode: WriteMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorename: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_groups: Option<Vec<PropertyGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_conflict: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadWriteFailed {
    pub reason: WriteError,
    pub upload_session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UploadError {
    Path(UploadWriteFailed),
    PropertiesError,
    PayloadTooLarge,
    ContentHashMismatch,
}

// files/upload_session/append_v2

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionAppendArg {
    pub cursor: UploadSessionCursor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionCursor {
    pub session_id: String,
    pub offset: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionAppendBatchArg {
    pub entries: Vec<UploadSessionAppendBatchEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionAppendBatchEntry {
    pub cursor: UploadSessionCursor,
    pub close: bool,
    pub length: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionAppendBatchResult {
    pub entries: Vec<UploadSessionAppendBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UploadSessionAppendBatchResultEntry {
    Success,
    Failure(UploadSessionAppendError),
}

// files/upload_session/finish

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionFinishArg {
    pub cursor: UploadSessionCursor,
    pub commit: CommitInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionFinishBatchArg {
    pub entries: Vec<UploadSessionFinishBatchEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionFinishBatchEntry {
    pub cursor: UploadSessionCursor,
    pub commit: CommitInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionFinishBatchResult {
    pub entries: Vec<UploadSessionFinishBatchResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UploadSessionFinishBatchResultEntry {
    Success(FileMetadataV2),
    Failure(UploadSessionFinishError),
}

// files/upload_session/finish_batch/check

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UploadSessionFinishBatchJobStatus {
    InProgress,
    Complete(UploadSessionFinishBatchResult),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UploadSessionFinishBatchLaunch {
    AsyncJobId { async_job_id: String },
    Complete(UploadSessionFinishBatchResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollArg {
    pub async_job_id: String,
}

// files/upload_session/start

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionStartArg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_type: Option<UploadSessionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionStartResult {
    pub session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UploadSessionStartError {
    ConcurrentSessionDataNotAllowed,
    ConcurrentSessionCloseNotAllowed,
    PayloadTooLarge,
    ContentHashMismatch,
}

// files/upload_session/start_batch

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionStartBatchArg {
    pub num_sessions: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_type: Option<UploadSessionType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadSessionStartBatchResult {
    pub session_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum UploadSessionType {
    #[serde(rename = "sequential")]
    Sequential,
    #[serde(rename = "concurrent")]
    Concurrent,
}

// RelocationError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MoveIntoVaultError {
    #[serde(rename = "is_shared_folder")]
    IsSharedFolder,
    #[serde(rename = "other")]
    Other,
}

// MoveIntoFamilyError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MoveIntoFamilyError {
    #[serde(rename = "is_shared_folder")]
    IsSharedFolder,
    #[serde(rename = "other")]
    Other,
}

// DeleteArg

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteArg {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_rev: Option<String>,
}

// DeleteError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
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

// DownloadError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum DownloadError {
    Path(LookupError),
    UnsupportedFile,
    Other,
}

// DownloadZipError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum DownloadZipError {
    Path(LookupError),
    TooLarge,
    TooManyFiles,
    Other,
}

// ExportError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ExportError {
    Path(LookupError),
    NonExportable,
    InvalidExportFormat,
    RetryError,
    Other,
}

// PreviewError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PreviewError {
    Path(LookupError),
    InProgress,
    UnsupportedExtension,
    UnsupportedContent,
    Other,
}

// GetTemporaryLinkError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum GetTemporaryLinkError {
    Path(LookupError),
    EmailNotVerified,
    UnsupportedFile,
    NotAllowed,
    Other,
}

// ListFolderError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ListFolderError {
    Path(LookupError),
    Other,
}

// ListFolderLongpollError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ListFolderLongpollError {
    Reset,
    Other,
}

// ListRevisionsError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ListRevisionsError {
    Path(LookupError),
    Other,
}

// SearchError

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SearchError {
    Path(LookupError),
    InvalidArgument(Option<String>),
    InternalError,
    Other,
}
