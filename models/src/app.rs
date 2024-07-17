use anyhow::Result;
use api::{anyhow, ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service, SyncClient};

use serde::Deserialize;

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::{self, Utils};

/// This endpoint performs App Authentication, validating the supplied app key and secret, and returns the supplied string, to allow you to test your code and connection to the Dropbox API. It has no other effect. If you receive an HTTP 200 response with the supplied query, it indicates at least part of the Dropbox API infrastructure is working and that the app key and secret valid.
pub struct AppRequest<'a> {
    access_token: &'a str,
    data: &'a str,
}

/// Response to validation of access token
#[derive(Deserialize, Debug)]
pub struct AppResponse {
    result: String,
}

impl utils::Utils for AppRequest<'_> {
    fn parameters(&self) -> impl serde::Serialize + Deserialize {
        let mut parameters: HashMap<&str, &str> = HashMap::new();
        parameters.insert("query", self.data);
        parameters
    }
}

/// Implementation of Service trait that provides functions related to async and sync queries
impl Service<AppResponse, BoxFuture<'_, Result<AppResponse>>> for AppRequest<'_> {
    fn call(&self) -> Result<Pin<Box<dyn Future<Output = Result<AppResponse>> + Send>>> {
        let endpoint = Endpoint::AppPost.get_endpoint_url();

        let response = AsyncClient
            .post(endpoint)
            .bearer_auth(self.access_token)
            .json(&self.parameters())
            .send();
        let block = async {
            let response = response
                .await
                .map_err(|err| ApiError::RequestError(err.into()))?;

            let response = response
                .error_for_status()
                .map_err(|err| ApiError::DropBoxError(err.into()))?;

            let response: AppResponse = response
                .json()
                .await
                .map_err(|err| ApiError::ParsingError(err.into()))?;

            Result::<AppResponse>::Ok(response)
        };
        Ok(Box::pin(block))
    }
    fn call_sync(&self) -> Result<AppResponse> {
        let endpoint = Endpoint::AppPost.get_endpoint_url();

        let response = SyncClient
            .post(endpoint)
            .bearer_auth(self.access_token)
            .header(
                Headers::ContentTypeAppJson.get_str().0,
                Headers::ContentTypeAppJson.get_str().1,
            )
            .json(&self.parameters())
            .send()
            .map_err(|err| ApiError::RequestError(err.into()))?;

        match response.error_for_status() {
            Ok(response) => {
                let response: AppResponse = response
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

    use super::{AppRequest, AppResponse};
    #[tokio::test]
    pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let data = "foo";
        let request = AppRequest { access_token, data };

        let f = request.call()?;
        let r = async { Result::<AppResponse>::Ok(tokio::spawn(f).await??) }.await?;
        println!("{:#?}", r);

        Ok(())
    }

    #[test]
    pub fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let data = "data";
        let request = AppRequest { access_token, data };

        let r = request.call_sync()?;

        println!("{:#?}", r);

        Ok(())
    }
}
