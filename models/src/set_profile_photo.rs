use anyhow::Result;
use api::{anyhow, ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service, SyncClient};

use serde::Deserialize;

use std::{collections::HashMap, future::Future, pin::Pin};

/// Sets a user's profile photo.
pub struct SetProfilePhotoRequest<'a> {
    access_token: &'a str,
    base64_data: &'a str,
}

/// Response struct for setting profile photo response
#[derive(Deserialize, Debug)]
pub struct SetProfilePhotoResponse {
    profile_photo_url: String,
}

/// Implementation of Service trait that provides functions related to async and sync queries
impl Service<SetProfilePhotoResponse, BoxFuture<'_, Result<SetProfilePhotoResponse>>>
    for SetProfilePhotoRequest<'_>
{
    fn call(
        &self,
    ) -> Result<Pin<Box<dyn Future<Output = Result<SetProfilePhotoResponse>> + Send>>> {
        let endpoint = Endpoint::SetProfilePhotoPost.get_endpoint_url();
        let mut payload: std::collections::HashMap<&str, std::collections::HashMap<&str, &str>> =
            std::collections::HashMap::new();
        let mut nested = HashMap::new();
        nested.insert(".tag", "base64_data");
        nested.insert("base64_data", self.base64_data);
        payload.insert("image", nested);
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

            let response: SetProfilePhotoResponse = response
                .json()
                .await
                .map_err(|err| ApiError::ParsingError(err.into()))?;

            Result::<SetProfilePhotoResponse>::Ok(response)
        };
        Ok(Box::pin(block))
    }
    fn call_sync(&self) -> Result<SetProfilePhotoResponse> {
        let endpoint = Endpoint::SetProfilePhotoPost.get_endpoint_url();
        let mut payload: std::collections::HashMap<&str, std::collections::HashMap<&str, &str>> =
            std::collections::HashMap::new();
        let mut nested = HashMap::new();
        nested.insert(".tag", "base64_data");
        nested.insert("base64_data", self.base64_data);
        payload.insert("image", nested);
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
                let response: SetProfilePhotoResponse = response
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

    use super::{SetProfilePhotoRequest, SetProfilePhotoResponse};
    #[tokio::test]
    pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let base64_data = "data";
        let request = SetProfilePhotoRequest {
            access_token,
            base64_data,
        };

        let f = request.call()?;
        let r = async {
            let r = tokio::spawn(f).await;
            let r = r?;
            let r = r?;

            Result::<SetProfilePhotoResponse>::Ok(r)
        }
        .await?;

        Ok(())
    }

    #[test]
    pub fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
        let access_token = "token";
        let base64_data = "data";
        let request = SetProfilePhotoRequest {
            access_token,
            base64_data,
        };

        let r = request.call_sync()?;

        println!("{:#?}", r);

        Ok(())
    }

    #[test]
    pub fn test_json_from_nested_hash_map() -> Result<()> {
        let body = r##"{\"photo\":{\".tag\":\"base64_data\",\"base64_data\":\"SW1hZ2UgZGF0YSBpbiBiYXNlNjQtZW5jb2RlZCBieXRlcy4gTm90IGEgdmFsaWQgZXhhbXBsZS4=\"}}"##;
        let mut nested = std::collections::HashMap::new();
        let mut payload: std::collections::HashMap<&str, std::collections::HashMap<&str, &str>> =
            std::collections::HashMap::new();
        nested.insert(".tag", "base64_data");
        nested.insert(
            "base64_data",
            "SW1hZ2UgZGF0YSBpbiBiYXNlNjQtZW5jb2RlZCBieXRlcy4gTm90IGEgdmFsaWQgZXhhbXBsZS4=",
        );
        payload.insert("photo", nested);

        let request = SyncClient
            .post("https://endpoint.com/get")
            .bearer_auth("token")
            .header("Content-Type", "application/json")
            .json(&payload)
            .build()?;

        println!("{:#?} {}", request.body().unwrap(), body);

        Ok(())
    }
}
