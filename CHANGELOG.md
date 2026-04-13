# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added (round two)
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
- `users` namespace: 5 endpoints (get_current_account, get_account,
  get_account_batch, get_space_usage, features/get_values) plus supporting
  types (FullAccount, BasicAccount, SpaceUsage, ...).
- `sharing` namespace skeleton: 2 of 44 endpoints (revoke_shared_link,
  list_shared_links). Module-level rustdoc inventories the remaining 42.
- `helpers::chunked_upload::upload_large_file()` — orchestrates
  upload_session/{start, append_v2, finish} for files over 150 MiB.
- `upload_session_append.rs` — was an empty stub; now implemented.
- `TemplateFilterBase` enum replaces `Option<Vec<String>>` for
  `include_property_groups` on list_folder / get_metadata / search args.

### Changed (round two)
- **Breaking**: `Service::call()` now returns `BoxFuture<'static, Result<...>>`
  directly instead of `Result<BoxFuture<...>>`. Callers go from
  `req.call()?.await?` to `req.call().await?`.
- `Headers::ContentTypeAppOctetStream` is now a parameterless marker; the
  previous `String` payload was dead code.
- Endpoint test modules gated with `#[cfg(all(test, feature = "test-utils"))]`
  so `cargo build --tests` works without the feature.

### Added (round one — retroactive)
- Complete `files` namespace — every v2 endpoint now has a working Request /
  Response model, fixture, and unit test (144 tests, 0 failures).
- `decode_dropbox_error` utility in `src/errors` — parses Dropbox's
  `{"error": ..., "error_summary": ...}` envelope on 4xx/5xx responses.
- `Headers::ContentTypeAppOctetStream` (parameterless marker for content
  endpoints) and `Headers::DropboxApiResult` (output-side marker for download
  endpoints).
- New types: `PollArg`, `UploadSessionFinishBatchLaunch`.
- Missing error enums: `DownloadError`, `DownloadZipError`, `ExportError`,
  `PreviewError`, `GetTemporaryLinkError`, `ListFolderError`,
  `ListFolderLongpollError`, `ListRevisionsError`, `SearchError`.
- `FolderSharingInfo`; completed field coverage on `FolderMetadata`.
- `tests/live_dropbox.rs` — env-gated real-server integration test scaffold
  (skips without `DROPBOX_TEST_TOKEN`).
- Crate-level rustdoc with a `list_folder` quickstart.
- GitHub Actions CI: build, test, fmt, clippy.

### Changed
- `implement_service!` macro: non-2xx responses are now decoded through the
  Dropbox error envelope instead of being discarded.
- Content endpoints (`upload*`, `download*`, etc.) no longer double-send the
  args payload as a JSON body — args travel exclusively in the
  `Dropbox-API-Arg` header.
- Download-class endpoints parse `Dropbox-API-Result` response header for
  metadata instead of trying to deserialise the binary body as JSON.
- Mock-server plumbing fixed: host switched from `0.0.0.0` → `127.0.0.1`,
  path-index slicing made dynamic.
- Public `files` namespace fully re-exported under `api::files`.
- Endpoint test modules now gated with `#[cfg(all(test, feature = "test-utils"))]`
  so `cargo build --tests` works without the feature.

### Fixed
- `SearchMatchTypeV2` serde tag (was `tag = "match_type"`; should be
  `tag = ".tag"`) + missing `rename_all = "snake_case"`.
- `UploadSessionFinishBatchJobStatus`, `UploadSessionFinishBatchResultEntry` —
  added `rename_all = "snake_case"` to match lowercase `.tag` values.
- `Args = FileMetadata` anti-pattern in 6 upload/upload-session files — aliases
  now point at the correct Arg and Result types.
- `FilesUploadPost` request fixture `mode` field uses object form
  `{".tag": "add"}` to match the `WriteMode` enum shape.

### Removed
- `Headers::ContentTypeAppOctetStream(String)` payload field — variant is now
  parameterless. The payload string was never a real header value.
- Misleading `tests/integration_tests.rs` — it exercised only type
  construction against a fake token; replaced by `tests/live_dropbox.rs`.

## [0.1.1] - prior

Initial files-namespace work by the original author.
