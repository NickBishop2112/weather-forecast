use actix_web::{App, HttpServer};
use paperclip::actix::OpenApiExt;
use weather::routes::users::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .configure(configure)
            .with_json_spec_at("/api-doc/swagger.json")
            .with_swagger_ui_at("/swagger-ui")
            .build()
    })
    .bind(("127.0.0.1", 8080))?

    .run()
    .await
}