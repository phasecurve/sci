use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use configuration::{get_config, DbSettings};
use sci::{configuration, startup};
use startup::run;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health-check", &app.addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_to_newsletter_returns_200_for_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=mark%20gray&email=mark.gray@phasecurve.com";
    let response = client
        .post(&format!("{}/newsletters", &app.addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(r#"SELECT name, email FROM newsletters"#,)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to get a saved newsletter sub");

    assert_eq!(saved.email, "mark.gray@phasecurve.com");
    assert_eq!(saved.name, "mark gray")
}

#[tokio::test]
async fn subscribe_to_newsletter_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=mark", "missing email"),
        ("email=mark.gray@phasecurve.com", "missing name"),
        ("", "missing email and name"),
    ];

    for (invalid_body, err) in test_cases {
        let response = client
            .post(&format!("{}/newsletters", &app.addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to post request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didn't fail with 400 as expected, for the payload {}",
            err
        )
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let addr = format!("http://127.0.0.1:{}", port);
    let mut config = get_config().expect("Failed to get the config");
    config.db.db_name = Uuid::new_v4().to_string();
    let pool = configure_db(&config.db).await;
    let server = run(listener, pool.clone()).expect("Failed to bind addr");

    // Spawn in the background task then discard tokio will clean up when the runtime is shut down
    let _ = tokio::spawn(server);
    TestApp { addr, pool }
}

pub async fn configure_db(config: &DbSettings) -> PgPool {
    let mut conn = PgConnection::connect(&config.connection_string_no_name())
        .await
        .expect("Failed connecting to db");

    conn.execute(format!(r#"create database "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create db");

    let pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to db");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the test db.");

    pool
}

pub struct TestApp {
    pub addr: String,
    pub pool: PgPool,
}
