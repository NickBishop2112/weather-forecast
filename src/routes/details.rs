use crate::{
    config::settings::get_config, handlers::forecast::get_weather,
    services::http_client::HttpClient,
};
use actix_web::web::{self, ServiceConfig};
use log::error;
use reqwest::Client;
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig) {
    let http_client: Arc<dyn HttpClient> = Arc::new(Client::new());
    let client_data = web::Data::new(http_client);

    let app_config = match get_config() {
        Ok(cfg) => web::Data::new(cfg),
        Err(err) => {
            let error_message = format!("Failed to get config: {}", err);

            error!("{}", error_message);
            panic!("{}", error_message);
        }
    };

    cfg.app_data(client_data.clone())
        .app_data(app_config)
        .service(web::resource("/weather/{city}").route(web::get().to(get_weather)));
}
