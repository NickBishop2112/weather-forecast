use std::sync::Arc;

use paperclip::actix::web::{self,ServiceConfig};
use reqwest::Client;
use crate::{handlers::forecast::get_weather, services::http_client::HttpClient};

pub fn configure(cfg: &mut ServiceConfig) {
  
   let http_client: Arc<dyn HttpClient> = Arc::new(Client::new());
   let client_data = web::Data::new(http_client);

    cfg
        .app_data(client_data.clone())                           
        .service(web::resource("/weather").route(web::get().to(get_weather)));
}