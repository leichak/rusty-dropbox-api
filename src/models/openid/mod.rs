//! Dropbox `openid` namespace.
//!
//! One endpoint: `userinfo`. Returns OIDC-style user claims.

pub mod userinfo;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoResult {
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub email_verified: Option<bool>,
    #[serde(default)]
    pub iss: Option<String>,
    #[serde(default)]
    pub sub: Option<String>,
}
