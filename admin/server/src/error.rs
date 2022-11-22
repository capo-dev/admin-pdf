use actix_web::{HttpResponse, ResponseError};
use admin::Error as AdminError;

#[derive(Debug)]
pub struct Error(AdminError);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::InternalServerError().json(self.0.clone())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        let url = err.url().map(|url| url.to_string());
        Error(AdminError::Reqwest {
            url,
            message: err.to_string(),
        })
    }
}
