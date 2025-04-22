use actix_web::{App, test};
use weather::config::settings::init_config;
use weather::routes::details::configure;

#[actix_rt::test]
async fn test_get_weather() {
    env_logger::init();

    // Arrange
    init_config().unwrap();

    let app = test::init_service(App::new().configure(configure)).await;

    // Act
    let req = test::TestRequest::get().uri("/weather/London").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 200);
}
