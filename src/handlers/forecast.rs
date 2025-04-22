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
    city: web::Path<String>, // #[utoipa::path( query, description = "City name", example = "London" )]
    client: web::Data<Arc<dyn HttpClient>>,
    config_provider: web::Data<Arc<dyn ConfigProvider>>,
) -> Result<HttpResponse, Error> {
    info!("Start Get weather forecast");
    debug!("City: {}", city);

    let config = match config_provider.get_config() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprint!("Failed to get config: {}", err);
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
                error!("Serializaion error: {}", err);
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
mod tests {
    use crate::{
        Error,
        config::settings::{AppConfig, MockConfigProvider},
        services::http_client::MockHttpClient,
    };

    use super::*;
    use actix_web::web;
    use mockall::predicate;
    use serde_json::json;

    #[actix_web::test]
    async fn test_get_weather_success() {
        // Arrange
        let config_provider = mock_config_provider();

        let client = mock_client("London", move |_| {
            Ok(json!({"weather": "sunny"}).to_string().clone())
        });

        // Act
        let response = get_weather("London".to_string().into(), client, config_provider).await;

        // Assert
        assert_eq!(response.unwrap().status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_weather_failure() {
        // Arrange
        let config_provider = mock_config_provider();

        let client = mock_client("London", |_| {
            Err(Error::NetworkError {
                message: "Network error".to_string(),
            })
        });

        // Act
        let response = get_weather("London".to_string().into(), client, config_provider).await;

        // Assert
        assert_eq!(
            response.unwrap().status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    fn mock_config_provider() -> web::Data<Arc<dyn ConfigProvider>> {
        let mut mock_config_provider = MockConfigProvider::new();

        let fake_config = Box::leak(Box::new(AppConfig {
            openweather_api_key: "test_key".to_string(),
        }));

        mock_config_provider
            .expect_get_config()
            .returning(|| Ok(fake_config));

        web::Data::new(Arc::new(mock_config_provider) as Arc<dyn ConfigProvider>)
    }

    fn mock_client(
        city: &str,
        response: impl Fn(&str) -> Result<String, Error> + Send + 'static,
    ) -> web::Data<Arc<dyn HttpClient>> {
        let mut mock_client = MockHttpClient::new();

        mock_client
            .expect_get()
            .with(predicate::str::contains(city))
            .returning(response);

        web::Data::new(Arc::new(mock_client) as Arc<dyn HttpClient>)
    }
}
