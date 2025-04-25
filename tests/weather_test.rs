use actix_web::{App, test};
use env::current_dir;
use serial_test::serial;
use std::env;
use std::error::Error;
use weather::config::settings::init_config;
use weather::routes::details::configure;

#[actix_rt::test]
async fn get_weather_forecasts() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Arrange
    init_config(current_dir()?).unwrap();

    let app = test::init_service(App::new().configure(configure)).await;

    // Act
    let req = test::TestRequest::get().uri("/weather/London").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 200);

    Ok(())
}

#[actix_rt::test]
#[serial]
#[should_panic(expected = "Config error: Configuration not initialized")]
async fn get_weather_forecastsa() {
    test::init_service(App::new().configure(configure)).await;
}
