/// Macro to implement tests for both synchronous and asynchronous API calls.
/// This macro generates test functions that mock API endpoints and verify the expected behavior of requests.
///
/// # Parameters:
/// - `$endpoint`: The API endpoint being tested.
/// - `$headers`: A vector of headers to include in the request.
/// - `$req`: The request type used in the test (struct representing the API request).
/// - `$payload`: The type of the payload being sent in the request.
#[macro_export]
macro_rules! implement_tests {
    ($endpoint:expr, $headers:expr, $req:ident, $payload:ty) => {
        #[tokio::test]
        pub async fn test_async() -> Result<(), Box<dyn std::error::Error>> {
            let (body, response) = get_endpoint_test_body_response($endpoint);

            let mut mock;
            {
                let mut server = get_mut_or_init_async().await;

                let url = get_endpoint_url($endpoint).2;

                let postfix_idx = if url.as_ref().unwrap().as_str().contains(".content") {
                    23
                } else if url.as_ref().unwrap().as_str().contains(".notify") {
                    22
                } else {
                    19
                };

                mock = server
                    .mock("POST", &url.unwrap().as_str()[postfix_idx..])
                    .with_status(200);

                let headers: Vec<Headers> = $headers;

                for h in &headers {
                    mock = mock.with_header(h.get_str().0, h.get_str().1);
                }
                if let Some(body) = &body {
                    mock = mock.match_body(mockito::Matcher::JsonString(body.to_string()));
                }

                if let Some(response) = &response {
                    mock = mock.with_body(response);
                }
                mock = mock.create_async().await;
            }

            let payload: Option<$payload>;
            if let Some(body) = body {
                payload = Some(serde_json::from_str(&body).expect("failed to deserialize"));
            } else {
                payload = None;
            }

            let request = $req {
                access_token: &TEST_AUTH_TOKEN,
                payload,
            };

            let f = request.call()?;
            let _ = f.await?;

            mock.assert();

            Ok(())
        }

        #[test]
        pub fn test_sync_pass() -> Result<(), Box<dyn std::error::Error>> {
            let (body, response) = get_endpoint_test_body_response($endpoint);

            let mut mock;
            {
                let mut server = get_mut_or_init();
                let url = get_endpoint_url($endpoint).1;

                let postfix_idx = if url.as_ref().unwrap().as_str().contains(".content") {
                    23
                } else if url.as_ref().unwrap().as_str().contains(".notify") {
                    22
                } else {
                    19
                };

                mock = server
                    .mock("POST", &url.unwrap().as_str()[postfix_idx..])
                    .with_status(200);

                let headers: Vec<Headers> = $headers;

                for h in &headers {
                    mock = mock.with_header(h.get_str().0, h.get_str().1);
                }
                if let Some(body) = &body {
                    mock = mock.match_body(mockito::Matcher::JsonString(body.to_string()));
                }
                if let Some(response) = &response {
                    mock = mock.with_body(response);
                }
                mock = mock.create();
            }

            let payload: Option<$payload>;
            if let Some(body) = body {
                payload = Some(serde_json::from_str(&body).expect("failed to deserialise"));
            } else {
                payload = None;
            }

            let request = $req {
                access_token: &TEST_AUTH_TOKEN,
                payload,
            };

            let _ = request.call_sync()?;
            mock.assert();

            Ok(())
        }
    };
}
