use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();

    let body = "name=john%20doe&email=john_doe%40gmail.com";
    let response = client
        .post(&format!("http://{}/subscriptions", app))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=john%20doe", "missing the email"),
        ("email=john_doe%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", app))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

async fn spawn_app() -> Result<String, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?.to_string();
    let router = quill::server();

    tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("Failed to start server");
    });

    Ok(address)
}
