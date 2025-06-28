use actix_web::{FromRequest, HttpRequest, dev::Payload};

use futures_util::future::{Ready, ready};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct CityQuery {
    pub names: Vec<String>,
}

impl FromRequest for CityQuery {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query = req.query_string();
        let names = query
            .split('&')
            .filter_map(|s| s.strip_prefix("names="))
            .map(|s| s.to_string())
            .collect();
        ready(Ok(CityQuery { names }))
    }
}

#[cfg(test)]
mod tests {
    use actix_web::test::TestRequest;

    use super::*;

    #[actix_web::test]
    async fn test_city_query_from_request() {
        // Arrange
        let req = TestRequest::get()
            .uri("/?names=London&names=Paris")
            .to_http_request();

        // Act
        let city_query: CityQuery = CityQuery::from_request(&req, &mut Payload::None)
            .await
            .unwrap();

        // Assert
        assert_eq!(
            city_query.names,
            vec!["London".to_string(), "Paris".to_string()]
        );
    }
}
