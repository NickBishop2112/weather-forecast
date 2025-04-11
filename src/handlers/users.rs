use actix_web::HttpResponse;
use paperclip::actix::{
    api_v2_operation,
    web::Json,
};

use crate::models::user::User;

#[api_v2_operation(
    summary = "Get user",
    responses(
        (status_code = 200, description = "Successfully retrieved user"),
    )
)]
pub async fn get_user() -> HttpResponse {
    let user = User { id: 1, name: "John".into() };
    HttpResponse::Ok().json(user)
}

#[api_v2_operation(
    summary = "Create user",
    request_body = User,
    responses(
        (status_code = 201, description = "Successfully created user"),
    )
)]
pub async fn create_user(user: Json<User>) -> HttpResponse {
    HttpResponse::Created().json(user.into_inner())
}
