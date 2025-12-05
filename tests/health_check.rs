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
