/// Macro implementing tests
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

                mock = server
                    .mock("POST", &url.unwrap().as_str()[19..])
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
                payload = Some(serde_json::from_str(&body).expect("failed to deserialise"));
            } else {
                payload = None;
            }

            let request = $req {
                access_token: &TEST_TOKEN,
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

                mock = server
                    .mock("POST", &url.unwrap().as_str()[19..])
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

            println!("MOCK {:?}", mock);

            let payload: Option<$payload>;
            if let Some(body) = body {
                payload = Some(serde_json::from_str(&body).expect("failed to deserialise"));
            } else {
                payload = None;
            }

            let request = $req {
                access_token: &TEST_TOKEN,
                payload,
            };

            let _ = request.call_sync()?;
            mock.assert();

            Ok(())
        }
    };
}
