use dropbox_api::api;

#[test]
fn create_requests() {
    let request = api::auth::token_revoke::TokenRevokeRequest {
        access_token: "12345",
        payload: None,
    };
}
