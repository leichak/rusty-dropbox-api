use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, reqwest,
    reqwest::{Request, Response},
    ApiError, AsyncClient, BoxFuture, Endpoint, Headers, Service, SyncClient,
};

use serde::{Deserialize, Serialize};
use serde_json;

use std::{future::Future, pin::Pin};

use crate::utils::Utils;

/// Name and Value
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
pub struct PropertiesAddResponse();

impl Utils for PropertiesAddRequest<'_> {
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
impl Service<PropertiesAddResponse, BoxFuture<'_, Result<Option<PropertiesAddResponse>>>>
    for PropertiesAddRequest<'_>
{
    fn call(
        &self,
    ) -> Result<Pin<Box<dyn Future<Output = Result<Option<PropertiesAddResponse>>> + Send>>> {
        let mut endpoint = get_endpoint_url(Endpoint::PropertiesAddPost).0;
        if let Some(url) = get_endpoint_url(Endpoint::PropertiesAddPost).1 {
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

            let text = response
                .text()
                .await
                .map_err(|err| ApiError::ParsingError(err.into()))?;

            if text.is_empty() {
                return Ok(None);
            }

            let response: PropertiesAddResponse =
                serde_json::from_str(&text).map_err(|err| ApiError::ParsingError(err.into()))?;

            Result::<Option<PropertiesAddResponse>>::Ok(Some(response))
        };
        Ok(Box::pin(block))
    }
    fn call_sync(&self) -> Result<Option<PropertiesAddResponse>> {
        let endpoint = get_endpoint_url(Endpoint::PropertiesAddPost).0;

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
                let text = response
                    .text()
                    .map_err(|err| ApiError::ParsingError(err.into()))?;

                if text.is_empty() {
                    return Ok(None);
                }

                let response: PropertiesAddResponse = serde_json::from_str(&text)
                    .map_err(|err| ApiError::ParsingError(err.into()))?;
                Ok(Some(response))
            }
            Err(err) => Err(ApiError::DropBoxError(err.into()).into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use api::{get_mut_or_init, get_mut_or_init_async, Service};
    use tokio;

    use crate::TEST_TOKEN;

    use super::PropertiesAddRequest;
    use api::{mockito, Headers};

    #[tokio::test]
    pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
        let mock;
        {
            let body = r##"{
                                "path": "/my_awesome/word.docx",
                                "property_groups": [
                                    {
                                        "fields": [
                                            {
                                                "name": "Security Policy",
                                                "value": "Confidential"
                                            }
                                        ],
                                        "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
                                    }
                                ]
                            }"##;

            let mut server = get_mut_or_init_async().await;
            mock = server
                .mock("POST", "/2/file_properties/properties/add")
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
                .create_async()
                .await;
        }

        let path = "/my_awesome/word.docx";
        let property_groups = vec![(
            vec![("Security Policy", "Confidential")],
            "ptid:1a5n2i6d3OYEAAAAAAAAAYa",
        )];
        let request = PropertiesAddRequest {
            access_token: &TEST_TOKEN,
            path,
            property_groups: property_groups,
        };
        let _ = request.call()?.await?;
        mock.assert();

        Ok(())
    }

    #[test]
    pub fn test_sync_pass() -> Result<(), Box<dyn std::error::Error>> {
        let mock;
        {
            let body = r##"{
                                "path": "/my_awesome/word.docx",
                                "property_groups": [
                                    {
                                        "fields": [
                                            {
                                                "name": "Security Policy",
                                                "value": "Confidential"
                                            }
                                        ],
                                        "template_id": "ptid:1a5n2i6d3OYEAAAAAAAAAYa"
                                    }
                                ]
                            }"##;

            let mut server = get_mut_or_init();
            mock = server
                .mock("POST", "/2/file_properties/properties/add")
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
                .create();
        }

        let path = "/my_awesome/word.docx";
        let property_groups = vec![(
            vec![("Security Policy", "Confidential")],
            "ptid:1a5n2i6d3OYEAAAAAAAAAYa",
        )];
        let request = PropertiesAddRequest {
            access_token: &TEST_TOKEN,
            path,
            property_groups: property_groups,
        };
        let _ = request.call_sync()?;
        mock.assert();

        Ok(())
    }
}
