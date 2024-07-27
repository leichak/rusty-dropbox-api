use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::{self, Utils};

/// Sets a user's profile photo.
/// https://www.dropbox.com/developers/documentation/http/documentation#account-set_profile_photo
pub struct SetProfilePhotoRequest<'a> {
    access_token: &'a str,
    base64_data: &'a str,
}

/// Response struct for setting profile photo response
#[derive(Deserialize, Debug)]
pub struct SetProfilePhotoResponse {
    profile_photo_url: String,
}

/// Implementation of trait for payload
impl utils::Utils for SetProfilePhotoRequest<'_> {
    fn payload(&self) -> Option<impl Serialize + Deserialize> {
        let mut payload: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
        let mut payload_nested: HashMap<&str, &str> = HashMap::new();
        payload_nested.insert("base64_data", self.base64_data);
        payload_nested.insert(".tag", "base64_data");
        payload.insert("photo", payload_nested);
        Some(payload)
    }
}

implement_service!(
    SetProfilePhotoRequest<'_>,
    SetProfilePhotoResponse,
    Endpoint::SetProfilePhotoPost,
    vec![Headers::ContentTypeAppJson]
);

#[cfg(test)]
mod tests {
    use crate::TEST_TOKEN;

    use super::SetProfilePhotoRequest;

    use anyhow::Result;
    use api::{get_mut_or_init, get_mut_or_init_async, mockito, Headers, Service};
    use tokio;

    #[tokio::test]
    pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
        let mock;
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
            mock = server
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

        mock.assert();

        Ok(())
    }

    #[test]
    pub fn test_sync_pass() -> Result<(), Box<dyn std::error::Error>> {
        let mock;
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
            mock = server
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
        mock.assert();

        Ok(())
    }
}
