use actix_web::{test, web, App};
use weather::{handlers::users::{create_user, get_user}, models::user::User};    

#[actix_rt::test]
async fn test_get_user() {
    // Arrange
    let app = test::init_service(App::new().route("/user", web::get().to(get_user))).await;

    // Act
    let req = test::TestRequest::get().uri("/user").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 200);
    let user: User = test::read_body_json(resp).await;

    assert_eq!(user, User { id: 1, name: "John".into() });        
}

#[actix_rt::test]
async fn test_create_user() {
    // Arrange
    let app = test::init_service(App::new().route("/user", web::post().to(create_user))).await;
    
    let new_user = User { id: 2, name: "Jane".into() };

    // Act
    let req = test::TestRequest::post()
        .uri("/user")
        .set_json(&new_user)
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 201);
    let created_user: User = test::read_body_json(resp).await;

    assert_eq!(created_user, User { id: 2, name: "Jane".into() });                        
}
