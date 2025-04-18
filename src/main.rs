use actix_web::{App, HttpServer};
use paperclip::actix::OpenApiExt;
use weather::routes::details::configure;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    env_logger::init();
  
    HttpServer::new(move || {
        App::new()            
            .wrap_api()
            .configure(configure)
            .with_json_spec_at("/api-doc/swagger.json")
            .with_swagger_ui_at("/swagger-ui")
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
