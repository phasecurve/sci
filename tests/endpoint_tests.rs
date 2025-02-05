#[tokio::test]

async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health-check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = sci::run().expect("Failed to bind address");
    // Spawn in the background task then discard tokio will clean up when the runtime is shut down
    let _ = tokio::spawn(server);
}
