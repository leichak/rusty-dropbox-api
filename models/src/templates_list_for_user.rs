use anyhow::Result;
use api::{anyhow, ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service, SyncClient};

use serde::{Deserialize, Serialize};

use std::{future::Future, pin::Pin};

use crate::utils::Utils;

/// Name and Value
type Field<'a> = (&'a str, &'a str);

/// Template id
type TemplateID<'a> = &'a str;

/// Removes all manually added contacts. You'll still keep contacts who are on your team or who you imported. New contacts will be added when you share.
/// Docs: https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-remove
pub struct TemplatesListForUserRequest<'a> {
    access_token: &'a str,
    path: &'a str,
    property_groups: Vec<(Vec<Field<'a>>, TemplateID<'a>)>,
}

/// No return values
#[derive(Deserialize, Debug)]
pub struct TemplatesListForUserResponse {}

impl Utils for TemplatesListForUserRequest<'_> {
    /// Function that tries to generate Serialize object from request data
    fn parameters(&self) -> impl serde::Serialize + Deserialize {
        #[derive(Serialize, Deserialize, Debug)]
        struct Field<'a> {
            name: &'a str,
            value: &'a str,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct PropertyGroup<'a> {
            fields: Vec<Field<'a>>,
            template_id: &'a str,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Payload<'a> {
            path: &'a str,
            property_groups: Vec<PropertyGroup<'a>>,
        }

        let mut p_gps = vec![];
        for (nv_vec, id) in self.property_groups.iter() {
            let f: Vec<Field> = nv_vec
                .iter()
                .map(|(n, v)| Field { name: n, value: v })
                .collect();
            let p_g = PropertyGroup {
                fields: f,
                template_id: id,
            };
            p_gps.push(p_g);
        }

        Payload {
            path: self.path,
            property_groups: p_gps,
        }
    }
}

/// Implementation of Service trait that provides functions related to async and sync queries
impl Service<TemplatesListForUserResponse, BoxFuture<'_, Result<TemplatesListForUserResponse>>>
    for TemplatesListForUserRequest<'_>
{
    fn call(
        &self,
    ) -> Result<Pin<Box<dyn Future<Output = Result<TemplatesListForUserResponse>> + Send>>> {
        let endpoint = Endpoint::TemplatesListForUserPost.get_endpoint_url();

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

            let response: TemplatesListForUserResponse = response
                .json()
                .await
                .map_err(|err| ApiError::ParsingError(err.into()))?;

            Result::<TemplatesListForUserResponse>::Ok(response)
        };
        Ok(Box::pin(block))
    }
    fn call_sync(&self) -> Result<TemplatesListForUserResponse> {
        let endpoint = Endpoint::TemplatesListForUserPost.get_endpoint_url();

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
                let response: TemplatesListForUserResponse = response
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
    use api::{
        serde_json::{json},
    };
    

    use crate::utils::Utils;

    use super::{Field, TemplateID, TemplatesListForUserRequest};
    // #[tokio::test]
    // pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
    //     let access_token = "token";
    //     let base64_data = "data";
    //     let request = SetProfilePhotoRequest {
    //         access_token,
    //         base64_data,
    //     };

    //     let f = request.call()?;
    //     let r = async { Result::<SetProfilePhotoResponse>::Ok(tokio::spawn(f).await??) }.await?;
    //     println!("{:#?}", r);
    //     Ok(())
    // }

    // #[test]
    // pub fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
    //     let access_token = "token";
    //     let base64_data = "data";
    //     let request = SetProfilePhotoRequest {
    //         access_token,
    //         base64_data,
    //     };

    //     let r = request.call_sync()?;

    //     println!("{:#?}", r);

    //     Ok(())
    // }

    #[test]
    pub fn test_properties_generation() -> Result<()> {
        let mut p_gs: Vec<(Vec<Field>, TemplateID)> = vec![];
        let f = vec![("name", "val")];
        p_gs.push((f, "id"));

        let request = TemplatesListForUserRequest {
            access_token: "123",
            path: "123",
            property_groups: p_gs,
        };

        let params = request.parameters();
        let params = json!(params);
        println!("{:?}", params.to_string());

        Ok(())
    }
}
