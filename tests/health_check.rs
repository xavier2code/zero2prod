//! test/health_check.rs

#[tokio::test]
async fn health_check_works() {
    // Arrage
    spawn_app().await.expect("Faild to spawn our app.");

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

async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await
}
