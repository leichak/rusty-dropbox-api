# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0]

### Added
- `LICENSE` (GPL-3.0) at repo root — required by crates.io and downstream
  license detectors.
- `Cargo.toml` metadata: `categories`, `documentation`, `rust-version`,
  expanded `keywords`.

### Changed
- **Breaking**: every type in `file_properties` renamed to match the
  Dropbox Stone IDL.
  - `Field` → `PropertyField`
  - `PathWithPropertyGroupsArgs` → `AddPropertiesArg`
    (with `OverwritePropertyGroupArg` as a type alias)
  - `PathWithTemplateIdsArgs` → `RemovePropertiesArg`
  - `PathWithUpdatePropertyGroupsArgs` → `UpdatePropertiesArg`
  - `Mode` → `PropertiesSearchMode`
  - `Query` → `PropertiesSearchQuery`
    (also: `logical_operator` field is now `pub`, was private)
  - `QueriesWithTemplateFilterArgs` → `PropertiesSearchArg`
  - `Match` → `PropertiesSearchMatch`
  - `MatchesWithPropertyGroupsResult` → `PropertiesSearchResult`
  - `CursorArgs` → `PropertiesSearchContinueArg`
  - `AddOrUpdateField` removed — Stone reuses `PropertyField`.
  - `UpdatePropertyGroup` → `PropertyGroupUpdate`
  - `FieldDescription` → `PropertyFieldTemplate`
  - `PropertyTemplateArgs` → `AddTemplateArg`
  - `TaggedFieldType` → `PropertyType`
  - `FieldWithTaggedType` → `PropertyFieldTemplateTagged`
  - `PropertyTemplateWithTaggedTypeResult` → `GetTemplateResult`
  - `TemplateIdsResult` → `ListTemplateResult`
  - `TemplateIdResult` → `AddTemplateResult` (with `UpdateTemplateResult`
    as a type alias)
  - `TemplateIdArgs` → `GetTemplateArg` (with `RemoveTemplateArg` as a
    type alias)
  - `AddField` removed — Stone reuses `PropertyFieldTemplate`.
  - `AddFieldsToTemplateArgs` → `UpdateTemplateArg`

## [0.7.1]

### Added
- Test for `sharing::get_shared_link_file` (download endpoint).
- `account::SetProfilePhotoError` enum (matches Stone spec).
- `Endpoint::OAuth2TokenPost` so the OAuth2 token URL goes through the same
  test-URL rewriter as every other endpoint. Lets unit tests intercept
  refresh calls with mockito.
- End-to-end test for `Client::call`: 401 → refresh via mocked
  `oauth2/token` → retry → 200, with assertion that the in-memory token
  updates.

### Changed
- `account::PhotoSourceArg` reshaped from a hand-rolled `untagged` enum
  with a manual `tag` field to the proper `tag = ".tag"` form. Wire shape
  unchanged.
- `auth/mod.rs` no longer hardcodes the OAuth2 token URL — uses
  `get_endpoint_url(Endpoint::OAuth2TokenPost)`.

## [0.7.0]

### Added
- Round-trip tests for all 39 sharing endpoints (was 2).
- `tests_utils::with_test_server_async` / `with_test_server_sync` helpers.

### Changed
- **Breaking**: `sharing::MemberSelector` and `sharing::InviteeInfo` use
  struct variants (`Email { email: String }`) instead of tuple variants
  of String. Tuple-of-newtype-String doesn't work with serde
  internally-tagged enums; the wire form was already
  `{".tag": "email", "email": "..."}`.
- **Breaking**: `sharing::update_folder_policy` Response type is now
  `SharedFolderMetadata` (was the wrong `ShareFolderLaunch`).
- Test infrastructure: replaced the shared-global mockito server
  (`OnceLock<Mutex<Server>>` per port) with per-test ephemeral
  `mockito::Server::new_async()` instances. A thread-local URL override
  consulted by `endpoints::test_url` lets each test point at its own
  random-port server. No more shared-mutex poison cascade; tests run in
  parallel safely. Total stable test count: 243 (was 171).
- `MOCK_SERVER_SYNC` / `MOCK_SERVER_ASYNC` statics and
  `get_mut_or_init` / `get_mut_or_init_async` removed.
- CI test runner reverted to default parallel (no `--test-threads=1`
  needed).

## [0.6.0]

### Removed
- **Breaking**: the `secondary_emails` and `seen_state` namespaces. They
  were added in 0.4.0 from training-data memory; the actual Dropbox
  endpoints live under `team/members/secondary_emails/*` (a Dropbox Business
  team-admin namespace, intentionally out of scope) and `seen_state` has
  no public routes at all (only the `PlatformType` union, used by
  `team_log`). They would have 404'd against real Dropbox.

### Added
- `errors::ApiError::Unauthorized(anyhow::Error)` — split from the generic
  `DropBox` variant so `Client::call` can detect 401 and refresh.
- 5 missing variants on `files::LookupError`: `not_file`, `not_folder`,
  `restricted_content`, `unsupported_content_type`, `locked`.
- 4 missing variants on `files::WriteError`: `no_write_permission`,
  `team_folder`, `operation_suppressed`, `too_many_write_operations`.
- `files::DownloadArg.rev: Option<String>` — was missing.
- `users::GroupCreation` and `users::SharedFolderBlanketLinkRestrictionPolicy`
  enums plus the two corresponding fields on `TeamSharingPolicies`.
- `users::TeamSpaceAllocation.user_within_team_space_used_cached: u64` —
  was missing.
- `sharing::RequestedLinkAccessLevel` enum (separate from `LinkAccessLevel`,
  used by `SharedLinkSettings.access`).
- `sharing::SharedLinkSettings.require_password: Option<bool>` — was missing.
- `sharing::AddMember` struct for `AddFolderMemberArg.members`.
- `sharing::RelinquishFolderMembershipArg` struct (replaces the prior
  `SharedFolderIdArg` reuse, adds the missing `leave_a_copy` field).
- Round-trip tests for `helpers::download_stream`, `helpers::upload_stream`,
  `helpers::chunked_upload::upload_large_file`.
- Behavioural tests for `implement_service!`: 429 retry, 5xx retry,
  401 → `ApiError::Unauthorized`.

### Changed
- **Breaking**: `users::PaperAsFilesValue` and `users::FileLockingValue` are
  now plain structs (`{ enabled: bool }`) per the Stone spec. They were
  modelled as tagged unions, which is the wrong wire shape.
- **Breaking**: `sharing::MemberSelector` and `sharing::InviteeInfo` use
  struct variants (`Email { email: String }`) instead of tuple variants.
  Tuple variants on internally-tagged enums don't work with serde when the
  inner type is a single String.
- **Breaking**: `sharing::ShareFolderArg`, `UpdateFolderPolicyArg`,
  `SetAccessInheritanceArg`, `AddFileMemberArgs`, `AddFolderMemberArg`,
  `UpdateFileMemberArgs`, `UpdateFolderMemberArg`, `ListFileMembersArg`,
  `ListFolderMembersArgs`, `SharedLinkSettings.access`: stringly-typed
  fields are now proper enum types (`AclUpdatePolicy`, `MemberPolicy`,
  `SharedLinkPolicy`, `ViewerInfoPolicy`, `AccessInheritance`,
  `AccessLevel`, `MemberAction`, `FolderAction`,
  `RequestedLinkAccessLevel`).
- **Breaking**: `sharing::LinkAccessLevel` no longer has a `Max` variant —
  spec only defines `viewer` and `editor` for the *response-side* type.
  `Max` and `Default` live on the new `RequestedLinkAccessLevel` for the
  request side.
- `helpers::download_stream` and `helpers::upload_stream` route through
  `endpoints::get_endpoint_url` so the test-utils mock-server URL rewriter
  intercepts.
- `.github/workflows/ci.yml` runs unit tests with `--test-threads=1` to
  keep the shared global mockito server stable.
- The `SharingListFileMembersBatchPost` request fixture had a missing
  opening `{` (parse error).
- All sharing fixture access-level/audience/requested-visibility values
  use the canonical object form `{".tag": "..."}` instead of the
  shorthand string form.

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
