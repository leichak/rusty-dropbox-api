/// Macro implementing Service trait
#[macro_export]
macro_rules! implement_service {
    ($req:ty, $resp:ident, $resp_payload:ty, $endpoints:expr, $headers:expr) => {
        impl Service<$resp, BoxFuture<'_, Result<Option<$resp>>>> for $req {
            fn call_sync(&self) -> Result<Option<$resp>> {
                let mut endpoint = get_endpoint_url($endpoints).0;
                if let Some(url) = get_endpoint_url($endpoints).1 {
                    endpoint = url;
                }

                let headers: Vec<Headers> = $headers;

                let mut response = SyncClient.post(endpoint).bearer_auth(self.access_token);

                for h in &headers {
                    response = response.header(h.get_str().0, h.get_str().1);
                }

                if let Some(payload) = self.payload() {
                    response = response.json(payload);
                }

                let response = response
                    .send()
                    .map_err(|err| ApiError::Request(err.into()))?;

                match response.error_for_status() {
                    Ok(response) => {
                        let text = response
                            .text()
                            .map_err(|err| ApiError::Parsing(err.into()))?;

                        if text.is_empty() {
                            return Ok(None);
                        }

                        let response: $resp_payload = serde_json::from_str(&text)
                            .map_err(|err| ApiError::Parsing(err.into()))?;
                        let response = $resp { payload: response };
                        Ok(Some(response))
                    }
                    Err(err) => Err(ApiError::DropBox(err.into()).into()),
                }
            }

            fn call(&self) -> Result<Pin<Box<dyn Future<Output = Result<Option<$resp>>> + Send>>> {
                let mut endpoint = get_endpoint_url($endpoints).0;
                if let Some(url) = get_endpoint_url($endpoints).2 {
                    endpoint = url;
                }

                let headers: Vec<Headers> = $headers;

                let mut response = AsyncClient.post(endpoint).bearer_auth(self.access_token);

                for h in &headers {
                    response = response.header(h.get_str().0, h.get_str().1);
                }

                if let Some(payload) = &self.payload() {
                    response = response.json(payload);
                }

                let response = response.send();

                let block = async {
                    let response = response
                        .await
                        .map_err(|err| ApiError::Request(err.into()))?;

                    let response = response
                        .error_for_status()
                        .map_err(|err| ApiError::DropBox(err.into()))?;

                    let text = response
                        .text()
                        .await
                        .map_err(|err| ApiError::Parsing(err.into()))?;

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