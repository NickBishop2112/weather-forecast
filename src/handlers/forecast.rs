use std::sync::Arc;

use crate::{services::http_client::HttpClient};
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
    app_config: web::Data<crate::config::settings::AppConfig>,
) -> Result<HttpResponse, Error> {
    info!("Start Get weather forecast");
    debug!("City: {}", city);

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, app_config.openweather_api_key
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
        config::settings::AppConfig,
        services::http_client::MockHttpClient,
    };

    use super::*;
    use actix_web::web::Data;
    use mockall::predicate;
    use serde_json::json;
    
    #[actix_web::test]
    async fn get_weather_successfully() {
        // Arrange
        let client = mock_http_client("London", move |_| {
            Ok(json!({"weather": "sunny"}).to_string().clone())
        }, 1);

        // Act
        let response = test_get_weather(client).await;

        // Assert
        assert_eq!(response.unwrap().status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_weather_fails_with_network_error() {
        
        // Arrange
        let client = mock_http_client("London", |_| {
            Err(Error::NetworkError {
                message: "Network error".to_string(),
            })
        }, 1);

        // Act
        let response = test_get_weather(client).await;

        // Assert
        assert_eq!(
            response.unwrap().status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[actix_web::test]
    async fn get_weather_fails_with_json_error() {
        
        // Arrange
        let client = mock_http_client("London", move |_| {
            Ok(r#"{ "weather": "sunny }"#.to_string().clone())
        }, 1);

        // Act
        let response = test_get_weather(client).await;

        // Assert
        assert_eq!(
            response.unwrap().status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    async fn test_get_weather(http_client: MockHttpClient) -> Result<HttpResponse, actix_web::Error> {

        let cfg = AppConfig { openweather_api_key: "test_key".to_string() }; 
        get_weather("London".to_string().into(), Data::new(Arc::new(http_client) as Arc<dyn HttpClient>), web::Data::new(cfg)).await
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
