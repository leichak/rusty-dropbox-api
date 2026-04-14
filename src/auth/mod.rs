//! OAuth2 helpers for Dropbox.
//!
//! Dropbox issues short-lived access tokens (typically 4 hours). For any
//! long-running app you must request a *refresh token* by passing
//! `token_access_type=offline` to the authorize URL, then exchange it via
//! `oauth2/token` whenever the access token expires.
//!
//! See <https://developers.dropbox.com/oauth-guide> for the full flow.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::endpoints::{get_endpoint_url, Endpoint};

const AUTHORIZE_URL: &str = "https://www.dropbox.com/oauth2/authorize";

/// Resolve the OAuth2 token endpoint, honoring the test-utils URL override
/// so unit tests can intercept the call with mockito.
fn token_url() -> String {
    let (live, _sync_test, async_test) = get_endpoint_url(Endpoint::OAuth2TokenPost);
    async_test.unwrap_or(live)
}

fn token_url_sync() -> String {
    let (live, sync_test, _async_test) = get_endpoint_url(Endpoint::OAuth2TokenPost);
    sync_test.unwrap_or(live)
}

/// Token bundle returned by `exchange_code` and `refresh`.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    /// Present on initial exchange with `token_access_type=offline`; absent
    /// from refresh responses (the same refresh token stays valid).
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub uid: Option<String>,
}

/// Build the URL the user should visit to grant your app access. After they
/// approve, Dropbox redirects to your `redirect_uri` with `?code=...` which
/// you then hand to [`exchange_code`].
///
/// Pass `offline = true` to receive a `refresh_token` in the exchange step.
pub fn authorize_url(
    client_id: &str,
    redirect_uri: &str,
    state: Option<&str>,
    offline: bool,
) -> String {
    let mut url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}",
        AUTHORIZE_URL,
        urlencode(client_id),
        urlencode(redirect_uri),
    );
    if offline {
        url.push_str("&token_access_type=offline");
    }
    if let Some(s) = state {
        url.push_str("&state=");
        url.push_str(&urlencode(s));
    }
    url
}

/// Exchange the authorization `code` (from your redirect callback) for an
/// access token + refresh token.
pub async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<Tokens> {
    let form = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
    ];
    let resp = crate::AsyncClient
        .post(token_url())
        .form(&form)
        .send()
        .await
        .context("oauth2/token send failed")?
        .error_for_status()
        .context("oauth2/token returned non-2xx")?;
    resp.json().await.context("oauth2/token parse")
}

/// Get a fresh access token using a refresh token. The refresh token itself
/// stays valid across this call.
pub async fn refresh(client_id: &str, client_secret: &str, refresh_token: &str) -> Result<Tokens> {
    let form = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];
    let resp = crate::AsyncClient
        .post(token_url())
        .form(&form)
        .send()
        .await
        .context("oauth2/token refresh send failed")?
        .error_for_status()
        .context("oauth2/token refresh returned non-2xx")?;
    resp.json().await.context("oauth2/token refresh parse")
}

/// Synchronous variant of [`refresh`] for blocking callers.
pub fn refresh_sync(client_id: &str, client_secret: &str, refresh_token: &str) -> Result<Tokens> {
    let form = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];
    let resp = crate::SyncClient
        .post(token_url_sync())
        .form(&form)
        .send()
        .context("oauth2/token refresh send failed")?
        .error_for_status()
        .context("oauth2/token refresh returned non-2xx")?;
    resp.json().context("oauth2/token refresh parse")
}

/// Revoke the current access token. After this, both the access token and the
/// refresh token (if any) are invalid.
pub async fn revoke(access_token: &str) -> Result<()> {
    crate::AsyncClient
        .post("https://api.dropboxapi.com/2/auth/token/revoke")
        .bearer_auth(access_token)
        .send()
        .await
        .context("auth/token/revoke send failed")?
        .error_for_status()
        .context("auth/token/revoke returned non-2xx")?;
    Ok(())
}

/// Minimal URL form-encoding for the few characters that matter to OAuth2
/// query strings (space, `=`, `&`, `:`, `/`, `?`). Avoids pulling in a full
/// urlencoding crate for one call site.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorize_url_basic() {
        let url = authorize_url("my_id", "http://localhost/cb", None, false);
        assert!(url.contains("client_id=my_id"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("redirect_uri=http%3A%2F%2Flocalhost%2Fcb"));
        assert!(!url.contains("token_access_type"));
    }

    #[test]
    fn authorize_url_offline_with_state() {
        let url = authorize_url("id", "https://app.example/cb", Some("xyz"), true);
        assert!(url.contains("token_access_type=offline"));
        assert!(url.contains("state=xyz"));
    }
}
