//! Dropbox `seen_state` namespace (1 endpoint). Marks notifications as seen.

pub mod mark_seen;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkSeenArg {
    pub seen_events: Vec<serde_json::Value>,
}
