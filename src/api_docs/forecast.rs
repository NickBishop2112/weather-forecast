use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::handlers::forecast::get_weather),
    components(schemas(/* your schemas */)),
    tags((name = "Weather API", description = "Weather forecasting API"))
)]
pub struct ApiDoc;
