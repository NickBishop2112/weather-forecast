use std::sync::Arc;

use crate::{
    config::settings::{ConfigProvider, RealConfigProvider},
    handlers::forecast::get_weather,
    services::http_client::HttpClient,
};
//use paperclip::actix::web::{self, ServiceConfig};
use actix_web::web::{self, ServiceConfig};
use reqwest::Client;

pub fn configure(cfg: &mut ServiceConfig) {
    let http_client: Arc<dyn HttpClient> = Arc::new(Client::new());
    let client_data = web::Data::new(http_client);

    let config_provider: Arc<dyn ConfigProvider> = Arc::new(RealConfigProvider {});
    let config_provider_data = web::Data::new(config_provider);

    cfg.app_data(client_data.clone())
        .app_data(config_provider_data.clone())
        .service(web::resource("/weather/{city}").route(web::get().to(get_weather)));
}
