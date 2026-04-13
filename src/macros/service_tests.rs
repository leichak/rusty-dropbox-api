//! Behavioural tests for `implement_service!` paths that the per-endpoint
//! `implement_tests!` blocks don't exercise: retry on 429 and 5xx,
//! Unauthorized branching, typed-error downcast.

#![cfg(all(test, feature = "test-utils"))]

use crate::api;
use crate::api::Service;
use crate::tests_utils::get_mut_or_init_async;

const RAW_FILE_METADATA: &str = r#"{".tag":"file","name":"f.txt","id":"id:abc","client_modified":"2025-01-01T00:00:00Z","server_modified":"2025-01-01T00:00:00Z","rev":"r1","size":1,"path_lower":"/f.txt","path_display":"/f.txt","is_downloadable":true}"#;

fn build_get_metadata_request<'a>(token: &'a str) -> api::files::get_metadata::GetMetadataRequest<'a> {
    api::files::get_metadata::GetMetadataRequest {
        access_token: token,
        payload: Some(api::files::GetMetadataArgs {
            path: "/f.txt".to_string(),
            include_media_info: None,
            include_deleted: None,
            include_has_explicit_shared_members: None,
            include_property_groups: None,
        }),
    }
}

#[tokio::test]
async fn retries_on_429_then_succeeds() {
    let (m429, m200);
    {
        let mut server = get_mut_or_init_async().await;
        m429 = server
            .mock("POST", "/2/files/get_metadata")
            .with_status(429)
            .with_header("Retry-After", "1")
            .with_body("")
            .expect(1)
            .create_async()
            .await;
        m200 = server
            .mock("POST", "/2/files/get_metadata")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(RAW_FILE_METADATA)
            .expect(1)
            .create_async()
            .await;
    }
    let req = build_get_metadata_request("test");
    let resp = req.call().await.expect("retry path failed");
    assert!(resp.is_some());
    m429.assert();
    m200.assert();
}

#[tokio::test]
async fn retries_on_503_then_succeeds() {
    let (m503, m200);
    {
        let mut server = get_mut_or_init_async().await;
        m503 = server
            .mock("POST", "/2/files/get_metadata")
            .with_status(503)
            .with_body("")
            .expect(1)
            .create_async()
            .await;
        m200 = server
            .mock("POST", "/2/files/get_metadata")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(RAW_FILE_METADATA)
            .expect(1)
            .create_async()
            .await;
    }
    let req = build_get_metadata_request("test");
    let resp = req.call().await.expect("5xx retry failed");
    assert!(resp.is_some());
    m503.assert();
    m200.assert();
}

#[tokio::test]
async fn unauthorized_returns_typed_error_variant() {
    let mock401;
    {
        let mut server = get_mut_or_init_async().await;
        mock401 = server
            .mock("POST", "/2/files/get_metadata")
            .with_status(401)
            .with_body(
                r#"{"error_summary":"expired_access_token/.","error":{".tag":"expired_access_token"}}"#,
            )
            .expect(1)
            .create_async()
            .await;
    }
    let req = build_get_metadata_request("test");
    let err = req.call().await.expect_err("expected 401 error");
    let downcast = err.downcast_ref::<crate::errors::ApiError>();
    assert!(
        matches!(downcast, Some(crate::errors::ApiError::Unauthorized(_))),
        "expected ApiError::Unauthorized, got: {:?}",
        err
    );
    mock401.assert();
}
