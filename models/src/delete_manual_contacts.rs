// use anyhow::Result;
// use api::{anyhow, ApiError, AsyncClient, BoxFuture, Endpoint, Service, SyncClient};

// use serde::Deserialize;

// use std::{future::Future, pin::Pin};

// /// Removes manually added contacts from the given list.
// pub struct DeleteManualContactsRequest<'a> {
//     access_token: &'a str,
// }

// /// No return values
// #[derive(Deserialize, Debug)]
// pub struct DeleteManualContactsResponse {}

// /// Implementation of Service trait that provides functions related to async and sync queries
// impl Service<DeleteManualContactsResponse, BoxFuture<'_, Result<DeleteManualContactsResponse>>>
//     for DeleteManualContactsRequest<'_>
// {
//     fn call(
//         &self,
//     ) -> Result<Pin<Box<dyn Future<Output = Result<DeleteManualContactsResponse>> + Send>>> {
//         let endpoint = Endpoint::DeleteManualContactsPost.get_endpoint_url();
//         let response = AsyncClient
//             .post(endpoint)
//             .bearer_auth(self.access_token)
//             .send();
//         let block = async {
//             let response = response
//                 .await
//                 .map_err(|err| ApiError::RequestError(err.into()))?;

//             let response = response
//                 .error_for_status()
//                 .map_err(|err| ApiError::DropBoxError(err.into()))?;

//             let response: DeleteManualContactsResponse = response
//                 .json()
//                 .await
//                 .map_err(|err| ApiError::ParsingError(err.into()))?;

//             Result::<DeleteManualContactsResponse>::Ok(response)
//         };
//         Ok(Box::pin(block))
//     }
//     fn call_sync(&self) -> Result<DeleteManualContactsResponse> {
//         let endpoint = Endpoint::DeleteManualContactsPost.get_endpoint_url();
//         let response = SyncClient
//             .post(endpoint)
//             .bearer_auth(self.access_token)
//             .send()
//             .map_err(|err| ApiError::RequestError(err.into()))?;

//         match response.error_for_status() {
//             Ok(response) => {
//                 let response: DeleteManualContactsResponse = response
//                     .json()
//                     .map_err(|err| ApiError::ParsingError(err.into()))?;
//                 Ok(response)
//             }
//             Err(err) => Err(ApiError::DropBoxError(err.into()).into()),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {

//     use anyhow::Result;
//     use api::{Service};
//     use tokio;

//     use super::{DeleteManualContactsRequest, DeleteManualContactsResponse};
//     #[tokio::test]
//     pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
//         let access_token = "token";

//         let request = DeleteManualContactsRequest { access_token };

//         let f = request.call()?;
//         let r =
//             async { Result::<DeleteManualContactsResponse>::Ok(tokio::spawn(f).await??) }.await?;
//         println!("{:#?}", r);

//         Ok(())
//     }

//     #[test]
//     pub fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
//         let access_token = "token";

//         let request = DeleteManualContactsRequest { access_token };

//         let r = request.call_sync()?;

//         println!("{:#?}", r);

//         Ok(())
//     }
// }
