//! Dropbox `secondary_emails` namespace (3 endpoints).
//!
//! Lets end users manage additional email addresses attached to their
//! Dropbox account.

pub mod add;
pub mod delete;
pub mod resend_verification_emails;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddSecondaryEmailsArg {
    pub secondary_emails: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddSecondaryEmailsResult {
    pub results: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteSecondaryEmailsArg {
    pub emails_to_delete: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteSecondaryEmailsResult {
    pub results: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResendVerificationEmailArg {
    pub emails_to_resend: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResendVerificationEmailResult {
    pub results: Vec<serde_json::Value>,
}
