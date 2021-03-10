use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("read config filed error ")]
    ConfigReadFile(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

pub type ServerResult<T> = std::result::Result<T, ServerError>;

#[derive(Debug, serde::Serialize)]
pub struct Msg {
    code: u32,
    msg: &'static str,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    //#[error("duplicated field for {0}({1})")]
    //DuplicatedField(&'static str, &'static str),
    #[error("bcrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("customer error")]
    CustomerError(Msg),
    #[error("unknon error")]
    UnknowError,
}

#[inline]
pub fn customer_error(code: u32, msg: &'static str) -> ApiError {
    ApiError::CustomerError(Msg { code, msg })
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::UnknowError => HttpResponse::InternalServerError()
                .content_type(actix_web::http::header::ContentType::json())
                .body(r#"{"code":50001, "msg": "unknown error"}"#),
            Self::CustomerError(msg) => HttpResponse::BadRequest().json(msg),
            _ => {
                println!("{:?}", self);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
