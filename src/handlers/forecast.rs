use actix_web::HttpResponse;
use paperclip::actix::api_v2_operation;
use serde_json::Value;
use reqwest;
use crate::config::settings::CONFIG;

#[api_v2_operation(
    summary = "Get Weather Forecast",
    responses(
        (status_code = 200, description = "Successfully retrieved weather data"),
        (status_code = 500, description = "Weather API call failed"),
        (status_code = 400, description = "Invalid API request")
    )
)]
pub async fn get_weather() -> HttpResponse {    
    let city = "London";
    
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, CONFIG.openweather_api_key
    );

    // Call OpenWeather API using reqwest
    match reqwest::get(&url).await {
        Ok(resp) => {
            if resp.status().is_success() {
                // Read the response body and parse as JSON
                match resp.text().await {
                    Ok(body) => match serde_json::from_str::<Value>(&body) {
                        Ok(weather) => HttpResponse::Ok().json(weather), // Return JSON response
                        Err(_) => HttpResponse::InternalServerError().body("Failed to parse JSON".to_string()),
                    },
                    Err(_) => HttpResponse::InternalServerError().body("Failed to read response body".to_string()),
                }
            } else {
                HttpResponse::BadRequest().body("Failed to fetch weather data".to_string())
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to call OpenWeather API".to_string()),
    }
}
