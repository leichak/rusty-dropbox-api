/// Macro to implement the `Service` trait for a given request type (`$req`) and response structure (`$resp`).
/// This macro supports both synchronous and asynchronous API calls.
///
/// # Parameters:
/// - `$req`: The request type, representing an API request with payload, headers, and endpoint.
/// - `$resp`: The response structure that wraps the actual payload returned by the API.
/// - `$resp_payload`: The type of the payload inside the response structure.
/// - `$endpoints`: The API endpoint URLs, which may vary depending on conditions (sync/async).
/// - `$headers`: A vector of headers to include in the request.
///
/// On non-2xx responses the response body is run through
/// `$crate::errors::decode_dropbox_error::<serde_json::Value>` which parses the
/// Dropbox `{"error": ..., "error_summary": ...}` envelope and includes both
/// fields in the returned `anyhow::Error`. A future variant of this macro can
/// accept a typed error parameter to enable `downcast_ref` pattern matching.
#[macro_export]
macro_rules! implement_service {
    ($req:ty, $resp:ident, $resp_payload:ty, $endpoints:expr, $headers:expr) => {
        impl Service<$resp, BoxFuture<'_, Result<Option<$resp>>>> for $req {
            // Synchronous call implementation
            fn call_sync(&self) -> Result<Option<$resp>> {
                let mut endpoint = get_endpoint_url($endpoints).0;
                if let Some(url) = get_endpoint_url($endpoints).1 {
                    endpoint = url;
                }

                let headers: Vec<Headers> = $headers;

                let mut response = SyncClient.post(endpoint).bearer_auth(self.access_token);

                let is_content_endpoint = headers
                    .iter()
                    .any(|h| matches!(h, Headers::ContentTypeAppOctetStream));

                for h in &headers {
                    match h {
                        Headers::DropboxApiArg(_) => {
                            let temp_h = Headers::DropboxApiArg(
                                serde_json::json!(self.payload().unwrap()).to_string(),
                            );
                            response = response.header(temp_h.get_str().0, temp_h.get_str().1);
                        }
                        _ => response = response.header(h.get_str().0, h.get_str().1),
                    }
                }

                if !is_content_endpoint {
                    if let Some(payload) = self.payload() {
                        response = response.json(payload);
                    }
                }

                let response = response
                    .send()
                    .map_err(|err| ApiError::Request(err.into()))?;

                let status = response.status();
                let text = response
                    .text()
                    .map_err(|err| ApiError::Parsing(err.into()))?;

                if !status.is_success() {
                    return Err(ApiError::DropBox(
                        $crate::errors::decode_dropbox_error::<serde_json::Value>(status, &text),
                    )
                    .into());
                }

                if text.is_empty() {
                    return Ok(None);
                }

                let response: $resp_payload = serde_json::from_str(&text)
                    .map_err(|err| ApiError::Parsing(err.into()))?;
                let response = $resp { payload: response };
                Ok(Some(response))
            }

            // Asynchronous call implementation
            fn call(&self) -> Result<Pin<Box<dyn Future<Output = Result<Option<$resp>>> + Send>>> {
                let mut endpoint = get_endpoint_url($endpoints).0;
                if let Some(url) = get_endpoint_url($endpoints).2 {
                    endpoint = url;
                }

                let headers: Vec<Headers> = $headers;

                let mut response = AsyncClient.post(endpoint).bearer_auth(self.access_token);

                let is_content_endpoint = headers
                    .iter()
                    .any(|h| matches!(h, Headers::ContentTypeAppOctetStream));

                for h in &headers {
                    match h {
                        Headers::DropboxApiArg(_) => {
                            let temp_h = Headers::DropboxApiArg(
                                serde_json::json!(self.payload().unwrap()).to_string(),
                            );
                            response = response.header(temp_h.get_str().0, temp_h.get_str().1);
                        }
                        _ => response = response.header(h.get_str().0, h.get_str().1),
                    }
                }

                if !is_content_endpoint {
                    if let Some(payload) = &self.payload() {
                        response = response.json(payload);
                    }
                }

                let response = response.send();

                let block = async {
                    let response = response
                        .await
                        .map_err(|err| ApiError::Request(err.into()))?;

                    let status = response.status();
                    let text = response
                        .text()
                        .await
                        .map_err(|err| ApiError::Parsing(err.into()))?;

                    if !status.is_success() {
                        return Err(ApiError::DropBox(
                            $crate::errors::decode_dropbox_error::<serde_json::Value>(
                                status, &text,
                            ),
                        )
                        .into());
                    }

                    if text.is_empty() {
                        return Ok(None);
                    }

                    let response: $resp_payload =
                        serde_json::from_str(&text).map_err(|err| ApiError::Parsing(err.into()))?;
                    let response = $resp { payload: response };

                    Result::<Option<$resp>>::Ok(Some(response))
                };
                Ok(Box::pin(block))
            }
        }
    };
}
