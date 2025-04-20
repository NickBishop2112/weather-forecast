use std::sync::Arc;

use actix_web::{web, HttpResponse};
use paperclip::actix::api_v2_operation;
use serde_json::Value;
use log::{error, info};

use crate::{config::settings::CONFIG, services::http_client::HttpClient};

#[api_v2_operation(
    summary = "Get Weather Forecast",
    responses(
        (status_code = 200, description = "Successfully retrieved weather data"),
        (status_code = 500, description = "Weather API call failed"),
        (status_code = 400, description = "Invalid API request")
    )
)]

pub async fn get_weather(client: web::Data<Arc<dyn HttpClient>>) -> HttpResponse {
    
    info!("Start Get weather forecast");
    
    let city = "London";
    
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, CONFIG.openweather_api_key
    );
    
    
    let get = client.get(&url);
    
    let response = match get.await {
        Ok(body) => match serde_json::from_str::<Value>(&body) {
            Ok(json) => HttpResponse::Ok().json(json),
            Err(err) => {
                error!("Serializaion error: {}", err);
                HttpResponse::InternalServerError().body("Failed to parse JSON")},
        },
        Err(err) => {
            eprintln!("Network Error: {}", err); // calls Display, which you already implemented
            HttpResponse::InternalServerError().body("Failed to call OpenWeather API")
        }
    };

    info!("Finish Get weather forecast");
    response
}


#[cfg(test)]
mod tests {
    use crate::{services::http_client::MockHttpClient, Error};
    
    use super::*;
    use actix_web::web;
    use mockall::predicate;
    use serde_json::json;
    
    #[actix_web::test]
    async fn test_get_weather_success() {
        let mut mock = MockHttpClient::new();
        let fake_response = json!({"weather": "sunny"}).to_string();

        mock.expect_get()
            .with(predicate::str::contains("London"))
            .returning(move|_| Ok(fake_response.clone()));

        let client = web::Data::new(Arc::new(mock) as Arc<dyn HttpClient>);
        let response = get_weather(client).await;

        assert_eq!(response.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_weather_failure() {
        let mut mock = MockHttpClient::new();
        mock.expect_get()
            .with(predicate::str::contains("London"))
            .returning(|_| Err(Error::NetworkError { message: "Network error".to_string() }));

        let client = web::Data::new(Arc::new(mock) as Arc<dyn HttpClient>);
        let response = get_weather(client).await;

        assert_eq!(response.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}