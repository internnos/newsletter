use std::net::TcpListener;
use zero2prod::startup::run;
use reqwest;


use sqlx::{PgConnection, Connection};
use zero2prod::configurations::get_configuration;


fn spawn_app(endpoint: &str) -> String {
    let listener = TcpListener::bind(format!("{endpoint}:0")).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("{endpoint}:{port}").to_string();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    address
}

#[tokio::test]
async fn health_check_works() {
    let endpoint = "127.0.0.1";
    let address = spawn_app(endpoint);
    println!("address: {address}");

    let client = reqwest::Client::new();

    let response = client
    .get(format!("http://{address}/health_check"))
    .send()
    .await
    .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let endpoint = "127.0.0.1";
    let address = spawn_app(endpoint);

    let client = reqwest::Client::new();

    let configurations = get_configuration().expect("Failed to read config file");
    let connection_string = configurations.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
    .await
    .expect("Failed to connect to postgres");


    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
    .post(format!("http://{address}/subscriptions"))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query("SELECT email, name FROM subscriptions")
    .fetch_one(&mut connection)
    .await
    .expect("Failed to fetch saved subscriptions");

    // assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    // assert_eq!(saved.name, "le guin");



}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    let endpoint = "127.0.0.1";
    let address = spawn_app(endpoint);

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, message) in test_cases {
        let response = client
        .post(format!("http://{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(
        400, 
        response.status().as_u16(),
        "The API did not fail with 400 bad request when the payload was {message}"
    );
    }

}