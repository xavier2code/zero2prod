//! test/health_check.rs

#[tokio::test]
async fn health_check_works() {
    // Arrage
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Faild to spawn our app.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Faild to spawn our app.");

    let _ = tokio::spawn(server);
}
