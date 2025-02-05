use std::net::TcpListener;

#[tokio::test]

async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health-check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failewd to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let server = sci::run(listener).expect("Failed to bind address");
    // Spawn in the background task then discard tokio will clean up when the runtime is shut down
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
