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
