use reqwest::Client;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let addr = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to send get request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

// launch our app in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener
        .local_addr()
        .expect("failed to get local address")
        .port();
    let server = tribune::run(listener).expect("Failed to bind the server to the address.");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
