use anyhow::Result;
use api::{anyhow, ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service, SyncClient};

use serde::Deserialize;

use std::{collections::HashMap, future::Future, pin::Pin};

/// Removes all manually added contacts. You'll still keep contacts who are on your team or who you imported. New contacts will be added when you share.
pub struct DeleteManualContactsBatchRequest<'a> {
    access_token: &'a str,
    email_addresses: &'a Vec<&'a str>,
}

/// Response struct for delete manual contacts
#[derive(Deserialize, Debug)]
pub struct DeleteManualContactsBatchResponse {}

/// Implementation of Service trait that provides functions related to async and sync queries
impl
    Service<
        DeleteManualContactsBatchResponse,
        BoxFuture<'_, Result<DeleteManualContactsBatchResponse>>,
    > for DeleteManualContactsBatchRequest<'_>
{
    fn call(
        &self,
    ) -> Result<Pin<Box<dyn Future<Output = Result<DeleteManualContactsBatchResponse>> + Send>>>
    {
        let endpoint = Endpoint::DeleteManualContactsBatchPost.get_endpoint_url();
        let mut payload: std::collections::HashMap<&str, &Vec<&str>> =
            std::collections::HashMap::new();
        payload.insert("email_addresses", self.email_addresses);
        let response = AsyncClient
            .post(endpoint)
            .bearer_auth(self.access_token)
            .header(
                Headers::ContentTypeAppJson.get_str().0,
                Headers::ContentTypeAppJson.get_str().1,
            )
            .json(&payload)
            .send();
        let block = async {
            let response = response
                .await
                .map_err(|err| ApiError::RequestError(err.into()))?;

            let response = response
                .error_for_status()
                .map_err(|err| ApiError::DropBoxError(err.into()))?;

            let response: DeleteManualContactsBatchResponse = response
                .json()
                .await
                .map_err(|err| ApiError::ParsingError(err.into()))?;

            Result::<DeleteManualContactsBatchResponse>::Ok(response)
        };
        Ok(Box::pin(block))
    }
    fn call_sync(&self) -> Result<DeleteManualContactsBatchResponse> {
        let endpoint = Endpoint::DeleteManualContactsBatchPost.get_endpoint_url();
        let mut payload: std::collections::HashMap<&str, &Vec<&str>> =
            std::collections::HashMap::new();
        payload.insert("email_addresses", self.email_addresses);
        let response = SyncClient
            .post(endpoint)
            .bearer_auth(self.access_token)
            .header(
                Headers::ContentTypeAppJson.get_str().0,
                Headers::ContentTypeAppJson.get_str().1,
            )
            .json(&payload)
            .send()
            .map_err(|err| ApiError::RequestError(err.into()))?;

        match response.error_for_status() {
            Ok(response) => {
                let response: DeleteManualContactsBatchResponse = response
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
    use api::{Service, SyncClient};
    use tokio;

    use super::{DeleteManualContactsBatchRequest, DeleteManualContactsBatchResponse};
    #[tokio::test]
    pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let email_addresses = vec!["@.com", "@#.com"];
        let request = DeleteManualContactsBatchRequest {
            access_token,
            email_addresses: &email_addresses,
        };

        let f = request.call()?;
        let r = async {
            let r = tokio::spawn(f).await;
            let r = r?;
            let r = r?;

            Result::<DeleteManualContactsBatchResponse>::Ok(r)
        }
        .await?;

        Ok(())
    }

    #[test]
    pub fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let email_addresses = vec!["@.com", "@#.com"];
        let request = DeleteManualContactsBatchRequest {
            access_token,
            email_addresses: &email_addresses,
        };

        let r = request.call_sync()?;

        println!("{:#?}", r);

        Ok(())
    }
}
