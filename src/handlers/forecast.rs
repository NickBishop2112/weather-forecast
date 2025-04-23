use std::sync::Arc;

use crate::{config::settings::ConfigProvider, services::http_client::HttpClient};
use actix_web::{Error, HttpResponse, web};
use log::{debug, error, info};
use serde_json::Value;

#[utoipa::path(
    get,
    path = "/weather/{city}",
    params(
        ("city" = String, Path, description = "City name to retrieve forecast", example = "London")
    ),
    responses(
        (status = 200, description = "Successfully retrieved weather data"),
        (status = 500, description = "Weather API call failed"),
        (status = 400, description = "Invalid API request")
    )
)]
pub async fn get_weather(
    city: web::Path<String>,
    client: web::Data<Arc<dyn HttpClient>>,
    config_provider: web::Data<Arc<dyn ConfigProvider>>,
) -> Result<HttpResponse, Error> {
    info!("Start Get weather forecast");
    debug!("City: {}", city);

    let config = match config_provider.get_config() {
        Ok(cfg) => cfg,
        Err(err) => {
            error!("Failed to get config: {}", err);
            return Ok(HttpResponse::InternalServerError().body("Failed to get config"));
        }
    };

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, config.openweather_api_key
    );

    let get = client.get(&url);

    let response = match get.await {
        Ok(body) => match serde_json::from_str::<Value>(&body) {
            Ok(json) => HttpResponse::Ok().json(json),
            Err(err) => {
                error!("Serialization error: {}", err);
                HttpResponse::InternalServerError().body("Failed to parse JSON")
            }
        },
        Err(err) => {
            eprintln!("Network Error: {}", err); // calls Display, which you already implemented
            HttpResponse::InternalServerError().body("Failed to call OpenWeather API")
        }
    };

    info!("Finish Get weather forecast");
    Ok(response)
}

#[cfg(test)]
mod expectation {
    use crate::{
        Error,
        config::settings::{AppConfig, MockConfigProvider},
        services::http_client::MockHttpClient,
    };

    use super::*;
    use actix_web::web::Data;
    use mockall::predicate;
    use serde_json::json;
    use crate::Error::ConfigError;
    
    #[actix_web::test]
    async fn get_weather_successfully() {
        // Arrange
        let config_provider = mock_config_provider(get_app_config(), 1);

        let client = mock_http_client("London", move |_| {
            Ok(json!({"weather": "sunny"}).to_string().clone())
        }, 1);

        // Act
        let response = test_get_weather(client, config_provider).await;

        // Assert
        assert_eq!(response.unwrap().status(), actix_web::http::StatusCode::OK);
       // config_provider.check();
    }
    
    #[actix_web::test]
    async fn get_weather_fails_with_network_error() {
        // Arrange
        let config_provider = mock_config_provider(get_app_config(), 1);
    
        let client = mock_http_client("London", |_| {
            Err(Error::NetworkError {
                message: "Network error".to_string(),
            })
        }, 1);
    
        // Act
        let response = test_get_weather(client, config_provider).await;
    
        // Assert
        assert_eq!(
            response.unwrap().status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[actix_web::test]
    async fn get_weather_fails_with_json_error() {
        // Arrange
        let config_provider = mock_config_provider(get_app_config(), 1);
        
        let client = mock_http_client("London", move |_| {
            Ok(r#"{ "weather": "sunny }"#.to_string().clone())
        }, 1);

        // Act
        let response = test_get_weather(client, config_provider).await;

        // Assert
        assert_eq!(
            response.unwrap().status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }
    
    #[actix_web::test]
    async fn get_weather_fails_with_missing_config() {
        // Arrange
        let config_provider = mock_config_provider(|| Err(ConfigError {
            message: "invalid config".to_string(),
        }) ,1);

        let client = mock_http_client("London", |_| {
            Err(Error::NetworkError {
                message: "Network error".to_string(),
            })
        }, 0);

        // Act
        let response = test_get_weather(client, config_provider).await;

        // Assert
        assert_eq!(
            response.unwrap().status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    async fn test_get_weather(http_client: MockHttpClient, config_provider: MockConfigProvider) -> Result<HttpResponse, actix_web::Error> {
        get_weather("London".to_string().into(), Data::new(Arc::new(http_client) as Arc<dyn HttpClient>), Data::new(Arc::new(config_provider) as Arc<dyn ConfigProvider>)).await
    }

    fn get_app_config() -> fn() -> Result<AppConfig, Error> {
        move || Ok(AppConfig {
            openweather_api_key: "test_key".to_string(),
        })
    }

    fn mock_config_provider(expectation:impl Fn() -> Result<AppConfig,Error> + Send + 'static, times: usize) -> MockConfigProvider {
        let mut mock_config_provider = MockConfigProvider::new();

        mock_config_provider
            .expect_get_config()
            .times(times)
            .returning(expectation);

        mock_config_provider
    }
    
    fn mock_http_client(
        city: &str,
        returning: impl Fn(&str) -> Result<String, Error> + Send + 'static, times: usize,
    ) -> MockHttpClient {
        let mut mock_client = MockHttpClient::new();

        mock_client
            .expect_get()
            .times(times)
            .with(predicate::str::contains(city))
            .returning(returning);

        mock_client
    }
}
