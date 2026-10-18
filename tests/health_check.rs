use reqwest::Client;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let addr = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        // use the server address our test helper returned
        .get(format!("{addr}/healthz"))
        .send()
        .await
        .expect("Failed to send get request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

// launch our app in the background
fn spawn_app() -> String {
    // create a listener
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    // then interrogate it's port
    let port = listener
        .local_addr()
        .expect("failed to get local address")
        .port();
    let server = tribune::run(listener).expect("Failed to bind the server to the address.");

    let _ = tokio::spawn(server);

    // use the port to return our fully formed address
    format!("http://127.0.0.1:{port}")
}
