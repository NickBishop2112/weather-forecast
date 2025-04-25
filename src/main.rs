use std::env;
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use weather::config::settings::init_config;
use weather::routes::details::configure; // Import ApiDoc from the correct module

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    init_config(env::current_dir()?)?;

    let openapi = weather::api_docs::forecast::ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .configure(configure)
            // Add Swagger UI endpoint
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
