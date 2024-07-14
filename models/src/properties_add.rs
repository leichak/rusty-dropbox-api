use anyhow::Result;
use api::{anyhow, ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service, SyncClient};

use serde::Deserialize;

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::utils::Utils;

/// Name Value
type Field<'a> = (&'a str, &'a str);

/// Template id
type TemplateID<'a> = &'a str;

/// Removes all manually added contacts. You'll still keep contacts who are on your team or who you imported. New contacts will be added when you share.
/// Docs: https://www.dropbox.com/developers/documentation/http/documentation#file_properties-properties-add
pub struct PropertiesAddRequest<'a> {
    access_token: &'a str,
    path: &'a str,
    property_groups: Vec<(Vec<Field<'a>>, TemplateID<'a>)>,
}

/// No return values
#[derive(Deserialize, Debug)]
pub struct PropertiesAddResponse {}

impl Utils for PropertiesAddRequest<'_> {
    // fn parameters(&self) -> impl serde::Serialize + Deserialize {
    //     struct Field
    //     struct Parameters<'a> {
    //         path: &'a str,
    //         property_groups: fields,
    //     }
    // }
}

/// Implementation of Service trait that provides functions related to async and sync queries
impl Service<PropertiesAddResponse, BoxFuture<'_, Result<PropertiesAddResponse>>>
    for PropertiesAddRequest<'_>
{
    fn call(&self) -> Result<Pin<Box<dyn Future<Output = Result<PropertiesAddResponse>> + Send>>> {
        let endpoint = Endpoint::PropertiesAddPost.get_endpoint_url();

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

            let response: PropertiesAddResponse = response
                .json()
                .await
                .map_err(|err| ApiError::ParsingError(err.into()))?;

            Result::<PropertiesAddResponse>::Ok(response)
        };
        Ok(Box::pin(block))
    }
    fn call_sync(&self) -> Result<PropertiesAddResponse> {
        let endpoint = Endpoint::PropertiesAddPost.get_endpoint_url();

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
                let response: PropertiesAddResponse = response
                    .json()
                    .map_err(|err| ApiError::ParsingError(err.into()))?;
                Ok(response)
            }
            Err(err) => Err(ApiError::DropBoxError(err.into()).into()),
        }
    }
}
