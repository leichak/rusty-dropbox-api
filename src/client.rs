//! Lightweight token holder.
//!
//! Today every request takes `access_token: &str` directly. `Client` is a
//! small ergonomic wrapper that lets callers keep the token once and hand
//! `client.token()` to each request. Future work may migrate to a builder
//! style on `Client` (`client.list_folder(...).call()`) — this type exists
//! so that change lands without another round of public-API churn.

#[derive(Debug, Clone)]
pub struct Client {
    token: String,
}

impl Client {
    /// Construct a client from a Dropbox OAuth2 access token.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    /// The token string, borrowed for passing into a `*Request::access_token` field.
    pub fn token(&self) -> &str {
        &self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_token() {
        let c = Client::new("abc");
        assert_eq!(c.token(), "abc");
    }

    #[test]
    fn accepts_string_and_str() {
        let _ = Client::new("abc".to_string());
        let _ = Client::new("abc");
    }
}
