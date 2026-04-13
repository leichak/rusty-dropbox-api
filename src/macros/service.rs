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
        impl Service<$resp> for $req {
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
                } else if let Some(data) = self.content_body() {
                    response = response.body(data.to_vec());
                }

                let mut attempts = 0u32;
                let response = loop {
                    let builder = response
                        .try_clone()
                        .ok_or_else(|| ApiError::Request(anyhow::anyhow!(
                            "request body not clonable; 429 retry unsupported"
                        )))?;
                    let r = builder
                        .send()
                        .map_err(|err| ApiError::Request(err.into()))?;
                    if r.status() != reqwest::StatusCode::TOO_MANY_REQUESTS
                        || attempts >= 3
                    {
                        break r;
                    }
                    let wait = r
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(1);
                    std::thread::sleep(std::time::Duration::from_secs(wait));
                    attempts += 1;
                };

                let status = response.status();
                let api_result_header = response
                    .headers()
                    .get("Dropbox-API-Result")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());
                let text = response
                    .text()
                    .map_err(|err| ApiError::Parsing(err.into()))?;

                if !status.is_success() {
                    return Err(ApiError::DropBox(
                        $crate::errors::decode_dropbox_error::<serde_json::Value>(status, &text),
                    )
                    .into());
                }

                let is_download_endpoint = headers
                    .iter()
                    .any(|h| matches!(h, Headers::DropboxApiResult));

                let payload_source = if is_download_endpoint {
                    api_result_header.unwrap_or_default()
                } else {
                    text
                };

                if payload_source.is_empty() {
                    return Ok(None);
                }

                let response: $resp_payload = serde_json::from_str(&payload_source)
                    .map_err(|err| ApiError::Parsing(err.into()))?;
                let response = $resp { payload: response };
                Ok(Some(response))
            }

            // Asynchronous call implementation
            fn call(&self) -> Pin<Box<dyn Future<Output = Result<Option<$resp>>> + Send>> {
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
                } else if let Some(data) = self.content_body() {
                    response = response.body(data.to_vec());
                }

                let is_download_endpoint = headers
                    .iter()
                    .any(|h| matches!(h, Headers::DropboxApiResult));

                let block = async move {
                    let mut attempts = 0u32;
                    let response = loop {
                        let builder = response
                            .try_clone()
                            .ok_or_else(|| ApiError::Request(anyhow::anyhow!(
                                "request body not clonable; 429 retry unsupported"
                            )))?;
                        let r = builder
                            .send()
                            .await
                            .map_err(|err| ApiError::Request(err.into()))?;
                        if r.status() != reqwest::StatusCode::TOO_MANY_REQUESTS
                            || attempts >= 3
                        {
                            break r;
                        }
                        let wait = r
                            .headers()
                            .get("Retry-After")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(1);
                        tokio::time::sleep(std::time::Duration::from_secs(wait)).await;
                        attempts += 1;
                    };

                    let status = response.status();
                    let api_result_header = response
                        .headers()
                        .get("Dropbox-API-Result")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
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

                    let payload_source = if is_download_endpoint {
                        api_result_header.unwrap_or_default()
                    } else {
                        text
                    };

                    if payload_source.is_empty() {
                        return Ok(None);
                    }

                    let response: $resp_payload = serde_json::from_str(&payload_source)
                        .map_err(|err| ApiError::Parsing(err.into()))?;
                    let response = $resp { payload: response };

                    Result::<Option<$resp>>::Ok(Some(response))
                };
                Box::pin(block)
            }
        }
    };
}

/// Variant of `implement_service!` for download-class endpoints.
///
/// Identical to `implement_service!` except the Response struct must have
/// both `pub payload: $resp_payload` and `pub data: Vec<u8>` fields, and the
/// macro populates `data` with the raw response body bytes so callers can
/// access the downloaded file contents. Metadata continues to be parsed from
/// the `Dropbox-API-Result` header (gated by `Headers::DropboxApiResult`).
#[macro_export]
macro_rules! implement_download_service {
    ($req:ty, $resp:ident, $resp_payload:ty, $endpoints:expr, $headers:expr) => {
        impl Service<$resp> for $req {
            fn call_sync(&self) -> Result<Option<$resp>> {
                let mut endpoint = get_endpoint_url($endpoints).0;
                if let Some(url) = get_endpoint_url($endpoints).1 {
                    endpoint = url;
                }

                let headers: Vec<Headers> = $headers;
                let mut response = SyncClient.post(endpoint).bearer_auth(self.access_token);

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

                let response = response
                    .send()
                    .map_err(|err| ApiError::Request(err.into()))?;

                let status = response.status();
                let api_result_header = response
                    .headers()
                    .get("Dropbox-API-Result")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());
                let bytes = response
                    .bytes()
                    .map_err(|err| ApiError::Parsing(err.into()))?;

                if !status.is_success() {
                    let text = String::from_utf8_lossy(&bytes);
                    return Err(ApiError::DropBox(
                        $crate::errors::decode_dropbox_error::<serde_json::Value>(status, &text),
                    )
                    .into());
                }

                let header_json = api_result_header.unwrap_or_default();
                if header_json.is_empty() {
                    return Ok(None);
                }
                let payload: $resp_payload = serde_json::from_str(&header_json)
                    .map_err(|err| ApiError::Parsing(err.into()))?;
                Ok(Some($resp {
                    payload,
                    data: bytes.to_vec(),
                }))
            }

            fn call(&self) -> Pin<Box<dyn Future<Output = Result<Option<$resp>>> + Send>> {
                let mut endpoint = get_endpoint_url($endpoints).0;
                if let Some(url) = get_endpoint_url($endpoints).2 {
                    endpoint = url;
                }

                let headers: Vec<Headers> = $headers;
                let mut response = AsyncClient.post(endpoint).bearer_auth(self.access_token);

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

                let block = async move {
                    let mut attempts = 0u32;
                    let response = loop {
                        let builder = response
                            .try_clone()
                            .ok_or_else(|| ApiError::Request(anyhow::anyhow!(
                                "request body not clonable; 429 retry unsupported"
                            )))?;
                        let r = builder
                            .send()
                            .await
                            .map_err(|err| ApiError::Request(err.into()))?;
                        if r.status() != reqwest::StatusCode::TOO_MANY_REQUESTS
                            || attempts >= 3
                        {
                            break r;
                        }
                        let wait = r
                            .headers()
                            .get("Retry-After")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(1);
                        tokio::time::sleep(std::time::Duration::from_secs(wait)).await;
                        attempts += 1;
                    };

                    let status = response.status();
                    let api_result_header = response
                        .headers()
                        .get("Dropbox-API-Result")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
                    let bytes = response
                        .bytes()
                        .await
                        .map_err(|err| ApiError::Parsing(err.into()))?;

                    if !status.is_success() {
                        let text = String::from_utf8_lossy(&bytes);
                        return Err(ApiError::DropBox(
                            $crate::errors::decode_dropbox_error::<serde_json::Value>(
                                status, &text,
                            ),
                        )
                        .into());
                    }

                    let header_json = api_result_header.unwrap_or_default();
                    if header_json.is_empty() {
                        return Ok(None);
                    }
                    let payload: $resp_payload = serde_json::from_str(&header_json)
                        .map_err(|err| ApiError::Parsing(err.into()))?;
                    Result::<Option<$resp>>::Ok(Some($resp {
                        payload,
                        data: bytes.to_vec(),
                    }))
                };
                Box::pin(block)
            }
        }
    };
}
