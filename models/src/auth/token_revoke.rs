// use anyhow::Result;
// use api::{
//     anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
//     Headers, Service, SyncClient,
// };

// use serde::{Deserialize, Serialize};

// use std::{future::Future, pin::Pin};

// use crate::utils::{self, Utils};

// /// Token revoke
// /// https://www.dropbox.com/developers/documentation/http/documentation#auth-token-revoke
// #[derive(Debug, PartialEq)]
// pub struct TokenRevokeRequest<'a> {
//     access_token: &'a str,
// }

// /// Response struct for token revoke
// #[derive(Deserialize, Debug)]
// pub struct TokenRevokeResponse;

// /// Implementation of trait for payload
// impl utils::Utils for TokenRevokeRequest<'_> {
//     fn payload(&self) -> Option<impl Serialize + Deserialize> {
//         None::<()>
//     }
// }

// implement_service!(
//     TokenRevokeRequest<'_>,
//     TokenRevokeResponse,
//     Endpoint::TokenRevokePost,
//     vec![]
// );
