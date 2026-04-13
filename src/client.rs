//! Lightweight token holder with optional OAuth refresh support.
//!
//! Two construction paths:
//! - `Client::new(token)` — bring-your-own short-lived access token.
//! - `Client::with_refresh(client_id, client_secret, refresh_token)` — the
//!   client now knows how to mint fresh access tokens via Dropbox's
//!   `oauth2/token` refresh grant. Call `client.ensure_fresh()` (async) or
//!   `client.ensure_fresh_sync()` (blocking) before a request when you've
//!   been idle long enough that the access token might have expired.
//!
//! Note: the request macros still take `access_token: &str`, so callers must
//! pass `client.token()` per request. A future revision can let macros
//! transparently re-issue with a refreshed token on 401.

use anyhow::Result;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    token: RwLock<TokenState>,
    refresh: Option<RefreshConfig>,
}

#[derive(Debug)]
struct TokenState {
    access_token: String,
    expires_at: Option<Instant>,
}

#[derive(Debug, Clone)]
pub struct RefreshConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

impl Client {
    /// Construct a client with an existing short-lived access token. No
    /// refresh capability — once it expires (~4h after issue) every request
    /// returns 401 until you build a new client.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(Inner {
                token: RwLock::new(TokenState {
                    access_token: token.into(),
                    expires_at: None,
                }),
                refresh: None,
            }),
        }
    }

    /// Construct a client that knows how to refresh its own access token.
    /// Pass the OAuth app's client_id, client_secret, and a long-lived
    /// refresh token (acquired via [`crate::auth::exchange_code`] with
    /// `offline=true`).
    pub fn with_refresh(
        access_token: impl Into<String>,
        expires_in_secs: u64,
        cfg: RefreshConfig,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                token: RwLock::new(TokenState {
                    access_token: access_token.into(),
                    expires_at: Some(Instant::now() + Duration::from_secs(expires_in_secs)),
                }),
                refresh: Some(cfg),
            }),
        }
    }

    /// The current access token, borrowed for one request. Note: holds the
    /// read lock briefly; clone the returned String if you need to release it.
    pub fn token(&self) -> String {
        self.inner.token.read().unwrap().access_token.clone()
    }

    /// True if the access token has expired (or expiry is unknown). Always
    /// false for tokens registered via `Client::new` without an expiry.
    pub fn is_expired(&self) -> bool {
        match self.inner.token.read().unwrap().expires_at {
            Some(t) => Instant::now() >= t,
            None => false,
        }
    }

    /// Refresh the access token using the registered refresh config. No-op
    /// if no refresh config or token isn't expired yet.
    pub async fn ensure_fresh(&self) -> Result<()> {
        if !self.is_expired() {
            return Ok(());
        }
        let cfg = match &self.inner.refresh {
            Some(c) => c.clone(),
            None => return Ok(()),
        };
        let tokens = crate::auth::refresh(
            &cfg.client_id,
            &cfg.client_secret,
            &cfg.refresh_token,
        )
        .await?;
        let mut state = self.inner.token.write().unwrap();
        state.access_token = tokens.access_token;
        state.expires_at = Some(Instant::now() + Duration::from_secs(tokens.expires_in));
        Ok(())
    }

    /// Unconditionally mint a new access token via the refresh grant. No-op
    /// if the client was constructed without a `RefreshConfig`. Used by
    /// `call` when a 401 comes back despite our expiry clock saying the
    /// token was still fresh.
    pub async fn force_refresh(&self) -> Result<()> {
        let cfg = match &self.inner.refresh {
            Some(c) => c.clone(),
            None => return Ok(()),
        };
        let tokens = crate::auth::refresh(
            &cfg.client_id,
            &cfg.client_secret,
            &cfg.refresh_token,
        )
        .await?;
        let mut state = self.inner.token.write().unwrap();
        state.access_token = tokens.access_token;
        state.expires_at = Some(Instant::now() + Duration::from_secs(tokens.expires_in));
        Ok(())
    }

    /// Sync variant of [`force_refresh`].
    pub fn force_refresh_sync(&self) -> Result<()> {
        let cfg = match &self.inner.refresh {
            Some(c) => c.clone(),
            None => return Ok(()),
        };
        let tokens = crate::auth::refresh_sync(
            &cfg.client_id,
            &cfg.client_secret,
            &cfg.refresh_token,
        )?;
        let mut state = self.inner.token.write().unwrap();
        state.access_token = tokens.access_token;
        state.expires_at = Some(Instant::now() + Duration::from_secs(tokens.expires_in));
        Ok(())
    }

    /// Run an async closure that builds and executes a request, auto-
    /// refreshing the access token first if expired, and retrying once if
    /// the server returns 401. The closure takes the current token string
    /// and returns the future to execute.
    ///
    /// ```ignore
    /// client.call(|token| async move {
    ///     req::ListFolderRequest { access_token: &token, payload: Some(...) }
    ///         .call().await
    /// }).await?;
    /// ```
    pub async fn call<T, F, Fut>(&self, mut f: F) -> Result<T>
    where
        F: FnMut(String) -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        self.ensure_fresh().await?;
        let token = self.token();
        match f(token).await {
            Ok(v) => Ok(v),
            Err(e) => {
                // If the error is specifically an Unauthorized wrapped in the
                // ApiError enum, try one force-refresh + replay.
                let is_401 = e
                    .downcast_ref::<crate::errors::ApiError>()
                    .is_some_and(|api| matches!(api, crate::errors::ApiError::Unauthorized(_)));
                if !is_401 || self.inner.refresh.is_none() {
                    return Err(e);
                }
                self.force_refresh().await?;
                let token = self.token();
                f(token).await
            }
        }
    }

    /// Sync version of [`ensure_fresh`] for blocking callers.
    pub fn ensure_fresh_sync(&self) -> Result<()> {
        if !self.is_expired() {
            return Ok(());
        }
        let cfg = match &self.inner.refresh {
            Some(c) => c.clone(),
            None => return Ok(()),
        };
        let tokens = crate::auth::refresh_sync(
            &cfg.client_id,
            &cfg.client_secret,
            &cfg.refresh_token,
        )?;
        let mut state = self.inner.token.write().unwrap();
        state.access_token = tokens.access_token;
        state.expires_at = Some(Instant::now() + Duration::from_secs(tokens.expires_in));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_token() {
        let c = Client::new("abc");
        assert_eq!(c.token(), "abc");
        assert!(!c.is_expired());
    }

    #[test]
    fn with_refresh_marks_expiry() {
        let c = Client::with_refresh(
            "tok",
            0,
            RefreshConfig {
                client_id: "x".into(),
                client_secret: "y".into(),
                refresh_token: "r".into(),
            },
        );
        // expires_in=0 means already expired now.
        assert!(c.is_expired());
    }
}
