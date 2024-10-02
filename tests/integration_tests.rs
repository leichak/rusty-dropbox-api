// Run integration tests with:
// cargo test --test '*'
mod tests {
    use chrono::DateTime;
    use dropbox_api::api;
    use dropbox_api::api::file_requests::{CreateFileRequestArgs, FileRequestDeadline};
    use dropbox_api::api::Service;
    use tokio;

    #[test]
    fn call_sync_example() {
        let request = api::auth::token_revoke::TokenRevokeRequest {
            access_token: "12345",
            payload: None,
        };

        if let Ok(Some(result)) = Service::call_sync(&request) {
            println!("Result {:?} ", result);
        } else {
            println!("Connection not present");
        }
    }

    #[tokio::test]
    async fn call_async_example() {
        let request = api::auth::token_revoke::TokenRevokeRequest {
            access_token: "12345",
            payload: None,
        };

        if let Ok(ft) = request.call() {
            let result = ft.await;
            if let Ok(result) = result {
                println!("Result {:?} ", result);
            } else {
                println!("Connection not present");
            }
        }
    }

    #[tokio::test]
    async fn call_async_advanced_struct() {
        let request = api::file_requests::create::CreateFileRequest {
            access_token: "12345",
            payload: Some(CreateFileRequestArgs {
                title: "a".to_string(),
                destination: "b".to_string(),
                deadline: Some(FileRequestDeadline {
                    deadline: DateTime::from_timestamp_millis(23123).unwrap(),
                    allow_late_uploads: None,
                }),
                open: false,
                description: Some("desc".to_string()),
                video_project_id: None,
            }),
        };

        if let Ok(ft) = request.call() {
            let result = ft.await;
            if let Ok(result) = result {
                println!("Result {:?} ", result);
            } else {
                println!("Connection not present");
            }
        }
    }
}
