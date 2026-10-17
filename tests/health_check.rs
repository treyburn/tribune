use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to send get request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

// launch our app in the background
fn spawn_app() {
    let server = tribune::run().expect("Failed to bind the server to the address.");

    tokio::spawn(server);
}
