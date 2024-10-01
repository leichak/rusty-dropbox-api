use dropbox_api::api;
use dropbox_api::api::Service;

#[test]
fn call_request_sync() {
    let request = api::auth::token_revoke::TokenRevokeRequest {
        access_token: "12345",
        payload: None,
    };

    if let Ok(Some(result)) = Service::call_sync(&request) {
        println!("Result {:?} ", result);
    }
}
