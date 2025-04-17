use std::sync::Arc;

use actix_web::{test, web, App};
use reqwest::Client;
use weather::{handlers::forecast::get_weather, services::http_client::HttpClient};    

#[actix_rt::test]
async fn test_get_weather() {

    env_logger::init();

    // Arrange
    let http_client: Arc<dyn HttpClient> = Arc::new(Client::new());
    let client_data = web::Data::new(http_client);
    
    let app = test::init_service(
        App::new()
        .app_data(client_data.clone())
        .route("/weather", web::get().to(get_weather))).await;

    // Act
    let req = test::TestRequest::get().uri("/weather").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert    
    assert_eq!(resp.status(), 200);      
}