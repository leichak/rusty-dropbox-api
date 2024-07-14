use anyhow::Result;
use api::{anyhow, ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service, SyncClient};

use serde::Deserialize;

use std::{future::Future, pin::Pin};

/// Disables the access token used to authenticate the call. If there is a corresponding refresh token for the access token, this disables that refresh token, as well as any other access tokens for that refresh token.
pub struct TokenRevokeRequest<'a> {
    access_token: &'a str,
}

/// No return values
#[derive(Deserialize, Debug)]
pub struct TokenRevokeResponse {}

/// Implementation of Service trait that provides functions related to async and sync queries
impl Service<TokenRevokeResponse, BoxFuture<'_, Result<TokenRevokeResponse>>>
    for TokenRevokeRequest<'_>
{
    fn call(&self) -> Result<Pin<Box<dyn Future<Output = Result<TokenRevokeResponse>> + Send>>> {
        let endpoint = Endpoint::TokenRevokePost.get_endpoint_url();
        let response = AsyncClient
            .post(endpoint)
            .bearer_auth(self.access_token)
            .send();
        let block = async {
            let response = response
                .await
                .map_err(|err| ApiError::RequestError(err.into()))?;

            let response = response
                .error_for_status()
                .map_err(|err| ApiError::DropBoxError(err.into()))?;

            let response: TokenRevokeResponse = response
                .json()
                .await
                .map_err(|err| ApiError::ParsingError(err.into()))?;

            Result::<TokenRevokeResponse>::Ok(response)
        };
        Ok(Box::pin(block))
    }
    fn call_sync(&self) -> Result<TokenRevokeResponse> {
        let endpoint = Endpoint::TokenRevokePost.get_endpoint_url();
        let response = SyncClient
            .post(endpoint)
            .bearer_auth(self.access_token)
            .header(
                Headers::ContentTypeAppJson.get_str().0,
                Headers::ContentTypeAppJson.get_str().1,
            )
            .send()
            .map_err(|err| ApiError::RequestError(err.into()))?;

        match response.error_for_status() {
            Ok(response) => {
                let response: TokenRevokeResponse = response
                    .json()
                    .map_err(|err| ApiError::ParsingError(err.into()))?;
                Ok(response)
            }
            Err(err) => Err(ApiError::DropBoxError(err.into()).into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use api::Service;
    use tokio;

    use super::{TokenRevokeRequest, TokenRevokeResponse};
    #[tokio::test]
    pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let request = TokenRevokeRequest { access_token };

        let f = request.call()?;
        let r = async { Result::<TokenRevokeResponse>::Ok(tokio::spawn(f).await??) }.await?;
        println!("{:#?}", r);

        Ok(())
    }

    #[test]
    pub fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let request = TokenRevokeRequest { access_token };

        let r = request.call_sync()?;
        println!("{:#?}", r);

        Ok(())
    }
}
