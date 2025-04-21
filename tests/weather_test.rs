use std::sync::Arc;

use actix_web::{App, test, web};
use reqwest::Client;
use weather::config::settings::{ConfigProvider, init_config};
use weather::{
    config::settings::RealConfigProvider, handlers::forecast::get_weather,
    services::http_client::HttpClient,
};

#[actix_rt::test]
async fn test_get_weather() {
    env_logger::init();

    // Arrange
    init_config().unwrap();

    let http_client: Arc<dyn HttpClient> = Arc::new(Client::new());
    let client_data = web::Data::new(http_client);
    let config_provider: Arc<dyn ConfigProvider> = Arc::new(RealConfigProvider {});
    let config_provider_data = web::Data::new(config_provider);

    let app = test::init_service(
        App::new()
            .app_data(client_data.clone())
            .app_data(config_provider_data.clone())
            .route("/weather", web::get().to(get_weather)),
    )
    .await;

    // Act
    let req = test::TestRequest::get().uri("/weather").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 200);
}
