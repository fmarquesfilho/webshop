use crate::helpers::create_app;

#[actix_rt::test]
async fn ping_returns_200_and_no_body() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/ping", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
