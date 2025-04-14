use once_cell::sync::Lazy;
use dotenv::dotenv;

pub struct AppConfig {
    pub openweather_api_key: String,
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    dotenv().ok();

    AppConfig {
        openweather_api_key: std::env::var("OPENWEATHER_API_KEY")
            .expect("OPENWEATHER_API_KEY must be set"),
    }
});