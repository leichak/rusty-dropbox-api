use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service,
    SyncClient,
};

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::{self, Utils};

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

/// Implementation of trait for parameters payload
impl utils::Utils for SetProfilePhotoRequest<'_> {
    fn parameters(&self) -> impl Serialize + Deserialize {
        let mut parameters: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
        let mut nested: HashMap<&str, &str> = HashMap::new();
        nested.insert("base64_data", self.base64_data);
        nested.insert(".tag", "base64_data");
        parameters.insert("photo", nested);
        parameters
    }
}

/// Implementation of Service trait that provides functions related to async and sync queries
impl Service<SetProfilePhotoResponse, BoxFuture<'_, Result<SetProfilePhotoResponse>>>
    for SetProfilePhotoRequest<'_>
{
    fn call(
        &self,
    ) -> Result<Pin<Box<dyn Future<Output = Result<SetProfilePhotoResponse>> + Send>>> {
        let mut endpoint = get_endpoint_url(Endpoint::SetProfilePhotoPost).0;
        if let Some(url) = get_endpoint_url(Endpoint::SetProfilePhotoPost).1 {
            endpoint = url;
        }

        let response = AsyncClient
            .post(endpoint)
            .bearer_auth(self.access_token)
            .header(
                Headers::ContentTypeAppJson.get_str().0,
                Headers::ContentTypeAppJson.get_str().1,
            )
            .json(&self.parameters())
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
        let endpoint = get_endpoint_url(Endpoint::SetProfilePhotoPost).0;

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

    use api::{get_mut_or_init, get_mut_or_init_async, Service, SyncClient};
    use tokio;

    use crate::TEST_TOKEN;

    use super::{SetProfilePhotoRequest, SetProfilePhotoResponse};
    use api::Headers;

    use api::mockito;

    #[tokio::test]
    pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
        {
            let body = r##"{
                "photo": {
                ".tag": "base64_data",
                "base64_data": "SW1hZ2UgZGF0YSBpbiBiYXNlNjQtZW5jb2RlZCBieXRlcy4gTm90IGEgdmFsaWQgZXhhbXBsZS4="
                        }
            }"##;

            let response = r##"{
    "profile_photo_url": "https://dl-web.dropbox.com/account_photo/get/dbaphid%3AAAHWGmIXV3sUuOmBfTz0wPsiqHUpBWvv3ZA?vers=1556069330102&size=128x128"
}"##;

            let mut server = get_mut_or_init_async().await;
            server
                .mock("POST", "/2/account/set_profile_photo")
                .with_status(200)
                .with_header(
                    Headers::ContentTypeAppJson.get_str().0,
                    Headers::ContentTypeAppJson.get_str().1,
                )
                .with_header(
                    Headers::Authorization.get_str().0,
                    Headers::Authorization.get_str().1,
                )
                .match_body(mockito::Matcher::JsonString(body.to_string()))
                .with_body(response)
                .create_async()
                .await;
        }

        let base64_data =
            "SW1hZ2UgZGF0YSBpbiBiYXNlNjQtZW5jb2RlZCBieXRlcy4gTm90IGEgdmFsaWQgZXhhbXBsZS4=";
        let request = SetProfilePhotoRequest {
            access_token: &TEST_TOKEN,
            base64_data,
        };

        let f = request.call()?;
        let _ = f.await?;

        Ok(())
    }

    #[test]
    pub fn test_sync_pass() -> Result<(), Box<dyn std::error::Error>> {
        {
            let body = r##"{
                "photo": {
                ".tag": "base64_data",
                "base64_data": "SW1hZ2UgZGF0YSBpbiBiYXNlNjQtZW5jb2RlZCBieXRlcy4gTm90IGEgdmFsaWQgZXhhbXBsZS4="
                        }
            }"##;

            let response = r##"{
    "profile_photo_url": "https://dl-web.dropbox.com/account_photo/get/dbaphid%3AAAHWGmIXV3sUuOmBfTz0wPsiqHUpBWvv3ZA?vers=1556069330102&size=128x128"
}"##;

            let mut server = get_mut_or_init();
            server
                .mock("POST", "/2/account/set_profile_photo")
                .with_status(200)
                .with_header(
                    Headers::ContentTypeAppJson.get_str().0,
                    Headers::ContentTypeAppJson.get_str().1,
                )
                .with_header(
                    Headers::Authorization.get_str().0,
                    Headers::Authorization.get_str().1,
                )
                .match_body(mockito::Matcher::JsonString(body.to_string()))
                .with_body(response)
                .create();
        }

        let base64_data =
            "SW1hZ2UgZGF0YSBpbiBiYXNlNjQtZW5jb2RlZCBieXRlcy4gTm90IGEgdmFsaWQgZXhhbXBsZS4=";
        let request = SetProfilePhotoRequest {
            access_token: &TEST_TOKEN,
            base64_data,
        };

        let _ = request.call_sync()?;

        Ok(())
    }
}
