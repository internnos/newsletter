use std::net::TcpListener;

use zero2prod;
use reqwest;

fn spawn_app(endpoint: &str) -> String {
    let listener = TcpListener::bind(format!("{endpoint}:0")).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("{endpoint}:{port}").to_string();
    let server = zero2prod::run(listener).expect("Failed to bind address");
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