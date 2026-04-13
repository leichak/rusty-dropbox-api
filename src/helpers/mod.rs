//! High-level helpers built on top of the raw endpoint types.
//!
//! These are opt-in conveniences. They don't add new HTTP endpoints; they
//! orchestrate existing ones to deliver common multi-call workflows in a
//! single function call.

pub mod chunked_upload;
pub mod download_stream;
pub mod upload_stream;
