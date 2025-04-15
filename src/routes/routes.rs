use paperclip::actix::web::{self,ServiceConfig};
use crate::handlers::{forecast::get_weather, users::{create_user, get_user}};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg        
        .service(
            web::resource("/users")
                .route(web::get().to(get_user))
                .route(web::post().to(create_user))                
        ).service(web::resource("/weather").route(web::get().to(get_weather)));
}