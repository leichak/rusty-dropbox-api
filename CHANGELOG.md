# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0]

### Added
- `helpers::upload_stream::upload_stream(token, path, reader, mode)` — single
  `/files/upload` POST that streams the body from any `AsyncRead`. Pair with
  `chunked_upload::upload_large_file` for files over 150 MiB.
- Typed permission/action taxonomy for sharing:
  `MemberPermission`, `FolderPermission`, `FilePermission` (`action` + `allow`
  + optional `reason`), `MemberAction`, `FolderAction`, `FileAction`,
  `PermissionDeniedReason`.
- `SharedContentLinkMetadata`, `LinkPermission`, `LinkAction` — typed body
  for the `link_metadata` / `expected_link_metadata` fields on
  `SharedFileMetadata`.
- `SharedLinkPolicy`, `LinkAudienceDisallowedReason` enums replacing
  `serde_json::Value` holes.

### Changed
- **Breaking**: `UserMembershipInfo.permissions`, `GroupMembershipInfo.permissions`,
  `InviteeMembershipInfo.permissions` are now `Option<Vec<MemberPermission>>`
  (was `Option<Vec<serde_json::Value>>`).
- **Breaking**: `SharedFolderMetadata.permissions` is
  `Option<Vec<FolderPermission>>`, `.link_metadata` is
  `Option<SharedContentLinkMetadata>`.
- **Breaking**: `SharedFileMetadata.policy` is `FolderPolicy`,
  `.permissions` is `Option<Vec<FilePermission>>`,
  `.link_metadata` / `.expected_link_metadata` are
  `Option<SharedContentLinkMetadata>`.
- **Breaking**: `FolderPolicy.shared_link_policy` is `SharedLinkPolicy`.
- **Breaking**: `LinkAudienceOption.disallowed_reason` is
  `Option<LinkAudienceDisallowedReason>`.

## [0.4.0]

### Added
- `Client::call(|token| async {...})` wraps a request closure with auto
  `ensure_fresh()` and one-shot 401 retry-with-`force_refresh()`. Opt-in;
  `req.call().await?` still works for callers that manage tokens themselves.
- `Client::force_refresh()` / `force_refresh_sync()` — unconditional token
  refresh used by `call`'s 401 path.
- `errors::ApiError::Unauthorized(anyhow::Error)` — split out of the generic
  `DropBox` variant so 401s are programmatically detectable.
- `helpers::download_stream(token, path)` — returns parsed `FileMetadata`
  alongside a `Stream<Item = Result<Bytes>>` of body chunks. Avoids the
  `Vec<u8>` buffering done by the regular `DownloadRequest`.
- Fully typed sharing response trees: `LinkPermissions`, `VisibilityPolicy`,
  `LinkAudienceOption`, `ResolvedVisibility`,
  `SharedLinkAccessFailureReason`, `Team`, `TeamMemberInfo`, `UserInfo`,
  `GroupInfo`, `GroupType`, `GroupManagementType`, `InviteeInfo`,
  `AccessLevel`, `UserMembershipInfo`, `GroupMembershipInfo`,
  `InviteeMembershipInfo`, `AclUpdatePolicy`, `MemberPolicy`,
  `ViewerInfoPolicy`, `AccessInheritance`, `FolderPolicy`,
  `SharedFolderMetadata`, `SharedFileMetadata`. Swapped into
  `FileLinkMetadata`, `FolderLinkMetadata`, `ListFileMembersResult`,
  `ListFolderMembersResult`, `ListFoldersResult`, `ListReceivedFilesResult`.
### Changed
- **Breaking**: macros now return `ApiError::Unauthorized(...)` on HTTP 401
  instead of folding it into `ApiError::DropBox(...)`. Callers that match
  on the error variant need a new arm.
- `reqwest` feature set adds `stream` (needed by the streaming download
  helper); new dep `bytes = "1"`.

## [0.3.0]

### Added
- `auth` module with OAuth2 helpers: `authorize_url`, `exchange_code`,
  `refresh`, `refresh_sync`, `revoke`, `Tokens` struct.
- `Client::with_refresh(...)` — Client now optionally carries a
  `RefreshConfig` and exposes `ensure_fresh()` / `ensure_fresh_sync()` to
  re-mint the access token via Dropbox's `oauth2/token` refresh grant.
- `openid` namespace with the `userinfo` endpoint and `UserInfoResult`.
- Typed enums on `sharing::SharedLinkSettings`: `RequestedVisibility`,
  `LinkAudience`, `LinkAccessLevel` (replace `Option<String>` fields).
- Typed Response wrappers for 17 sharing endpoints:
  `ListFoldersResult`, `ListReceivedFilesResult`, `ListFileMembersResult`,
  `ListFolderMembersResult`, `ShareFolderLaunch`, `LaunchEmptyResult`,
  `JobStatus`, `ShareFolderJobStatus`, `RemoveMemberJobStatus`.
- `get_shared_link_file` now uses `implement_download_service!` — actually
  downloads bytes via `Dropbox-API-Result` header + body split.
- `User-Agent: rusty_dropbox_sdk/<version>` set on both clients.

### Changed
- **Breaking**: retry loop now retries on 5xx as well as 429, with
  exponential backoff (1s/2s/4s) when no `Retry-After` header is sent.
- **Breaking**: `Client::token()` returns `String` instead of `&str` so the
  internal RwLock guard isn't held across the request lifetime.
- **Breaking**: `sharing::SharedLinkSettings` fields are now typed enums,
  not `Option<String>`.

## [0.2.1]

### Added
- `sharing::SharedLinkMetadata` (file/folder), `FileLinkMetadata`,
  `FolderLinkMetadata`, `SharedLinkSettings`,
  `CreateSharedLinkWithSettingsArg`, `GetSharedLinkMetadataArg`,
  `ModifySharedLinkSettingsArgs`. Wired into the 4 shared-link endpoints.
- Typed Args for 27 more sharing endpoints (folders, file/folder members,
  list_*, check_*_job_status). `SharedFolderIdArg`, `ShareFolderArg`,
  `UnshareFolderArg`, `TransferFolderArg`, `UpdateFolderPolicyArg`,
  `SetAccessInheritanceArg`, `ListFoldersArgs`, `ListFoldersContinueArg`,
  `MemberSelector`, `AddFileMemberArgs`, `AddFolderMemberArg`,
  `RemoveFileMemberArg`, `RemoveFolderMemberArg`, `UpdateFileMemberArgs`,
  `UpdateFolderMemberArg`, `UnshareFileArg`, `GetFileMetadataArg`,
  `GetFileMetadataBatchArg`, `GetFolderMetadataArg`, `ListFileMembersArg`,
  `ListFileMembersBatchArg`, `ListFileMembersContinueArg`,
  `ListFolderMembersArgs`, `ListFolderMembersContinueArg`, `PollArg`.
- `files::UploadWriteFailed { reason, upload_session_id }` — replaces the
  wrong inner type on `UploadError::Path`.

### Changed
- All tagged-union enums in `files/mod.rs` carry `rename_all = "snake_case"`
  in addition to per-variant renames.
- `MediaInfo` now declares `tag = ".tag"`.
- `FileLockMetadata.is_lockholder` gains `skip_serializing_if`.
- `CreateSharedLinkWithSettingsPost` request fixture: add the missing
  opening `{`.

## [0.2.0]

### Added
- `Client` token holder at crate root; `prelude` module for ergonomic imports.
- Binary body on 5 upload Request structs (`pub data: Option<Vec<u8>>`) and
  on 5 download Response structs (`pub data: Vec<u8>`). Upload and download
  actually work now.
- `Utils::content_body()` trait method + `implement_content_upload_utils!`
  macro to plug upload data into the service macro body.
- `implement_download_service!` macro — constructs Response with raw body
  bytes from content-endpoint responses.
- `errors::TypedError<T>` wrapper + revised `decode_dropbox_error` so callers
  can `err.downcast_ref::<TypedError<UploadError>>()` for pattern matching.
- `list_folder::ListFolderRequest::collect_all()` — walks cursor pagination.
- 429 `Retry-After` aware retry loop in the service macro (3 retries).
- `users` namespace: 5 endpoints (get_current_account, get_account,
  get_account_batch, get_space_usage, features/get_values) plus supporting
  types (FullAccount, BasicAccount, SpaceUsage, ...).
- `sharing` namespace: all 39 endpoints (initially `serde_json::Value`
  payloads; typed Args followed in 0.2.1).
- `helpers::chunked_upload::upload_large_file()` — orchestrates
  upload_session/{start, append_v2, finish} for files over 150 MiB.
- `upload_session_append.rs` — was an empty stub; now implemented.
- `TemplateFilterBase` enum replaces `Option<Vec<String>>` for
  `include_property_groups` on list_folder / get_metadata / search args.
- `tests/live_dropbox.rs` — env-gated real-server integration test scaffold
  (skips without `DROPBOX_TEST_TOKEN`).
- Crate-level rustdoc with a `list_folder` quickstart.
- GitHub Actions CI: build, test, fmt, clippy.
- `decode_dropbox_error` utility — parses Dropbox's
  `{"error": ..., "error_summary": ...}` envelope on 4xx/5xx responses.
- `Headers::DropboxApiResult` (output-side marker for download endpoints).
- New types: `PollArg`, `UploadSessionFinishBatchLaunch`.
- Missing error enums: `DownloadError`, `DownloadZipError`, `ExportError`,
  `PreviewError`, `GetTemporaryLinkError`, `ListFolderError`,
  `ListFolderLongpollError`, `ListRevisionsError`, `SearchError`.
- `FolderSharingInfo`; completed field coverage on `FolderMetadata`.

### Changed
- **Breaking**: `Service::call()` now returns `BoxFuture<'static, Result<...>>`
  directly instead of `Result<BoxFuture<...>>`. Callers go from
  `req.call()?.await?` to `req.call().await?`.
- `Headers::ContentTypeAppOctetStream` is now a parameterless marker; the
  previous `String` payload was dead code.
- Endpoint test modules gated with `#[cfg(all(test, feature = "test-utils"))]`
  so `cargo build --tests` works without the feature.
- `implement_service!` macro: non-2xx responses are now decoded through the
  Dropbox error envelope instead of being discarded.
- Content endpoints no longer double-send the args payload as a JSON body.
- Download-class endpoints parse `Dropbox-API-Result` response header for
  metadata instead of trying to deserialise the binary body as JSON.
- Mock-server plumbing fixed: host switched from `0.0.0.0` → `127.0.0.1`,
  path-index slicing made dynamic.
- `CommitInfo.mode` typed as `WriteMode` (was `String`); `WriteMode` derives
  `Clone`.

### Fixed
- `SearchMatchTypeV2` serde tag (was `tag = "match_type"`; should be
  `tag = ".tag"`) + missing `rename_all = "snake_case"`.
- `UploadSessionFinishBatchJobStatus`, `UploadSessionFinishBatchResultEntry` —
  added `rename_all = "snake_case"` to match lowercase `.tag` values.
- `Args = FileMetadata` anti-pattern in 6 upload/upload-session files.
- `FilesUploadPost` request fixture `mode` field uses object form
  `{".tag": "add"}`.

### Removed
- `Headers::ContentTypeAppOctetStream(String)` payload field — variant is now
  parameterless.
- Misleading `tests/integration_tests.rs`.

## [0.1.1]

Initial files-namespace work by the original author.
