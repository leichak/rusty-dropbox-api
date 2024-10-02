// Run integration tests with:
// cargo test --test '*'

mod tests {
    use chrono::DateTime;
    use rusty_dropbox_sdk::api;
    use rusty_dropbox_sdk::api::file_requests::{CreateFileRequestArgs, FileRequestDeadline};
    use rusty_dropbox_sdk::api::Service;
    use tokio;

    #[test]
    fn call_sync_example() {
        let request = api::auth::token_revoke::TokenRevokeRequest {
            access_token: "12345",
            payload: None,
        };

        match Service::call_sync(&request) {
            Ok(Some(result)) => println!("Result: {:?}", result),
            _ => println!("Connection not present"),
        }
    }

    #[tokio::test]
    async fn call_async_example() {
        let request = api::auth::token_revoke::TokenRevokeRequest {
            access_token: "12345",
            payload: None,
        };

        if let Ok(future) = request.call() {
            match future.await {
                Ok(result) => println!("Result: {:?}", result),
                _ => println!("Connection not present"),
            }
        }
    }

    #[tokio::test]
    async fn call_async_advanced_example() {
        let request = api::file_requests::create::CreateFileRequest {
            access_token: "12345",
            payload: Some(CreateFileRequestArgs {
                title: "Request Title".to_string(),
                destination: "Request Destination".to_string(),
                deadline: Some(FileRequestDeadline {
                    deadline: DateTime::from_timestamp_millis(23123).unwrap(),
                    allow_late_uploads: None,
                }),
                open: false,
                description: Some("Request description".to_string()),
                video_project_id: None,
            }),
        };

        if let Ok(future) = request.call() {
            match future.await {
                Ok(result) => println!("Result: {:?}", result),
                _ => println!("Connection not present"),
            }
        }
    }
}
