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

#[tokio::test]
async fn subscribe_to_newsletter_returns_200_for_valid_data() {
    let app_ip = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=mark&email=mark.gray@phasecurve.com";
    let response = client
        .post(&format!("{}/newsletters", &app_ip))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failewd to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let server = sci::run(listener).expect("Failed to bind address");
    // Spawn in the background task then discard tokio will clean up when the runtime is shut down
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
