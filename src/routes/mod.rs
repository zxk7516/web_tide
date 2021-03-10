use crate::error::ApiError;
use actix_web::{get, web, HttpResponse};
mod auth;
mod middleware;

#[macro_export]
macro_rules! dump {
    ($e:expr) => {
        if cfg!(debug_assertions) {
            dbg!($e)
        } else {
            $e
        }
    };
}

#[get("/ping")]
async fn hello() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("pong"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(auth::reg);
    cfg.service(auth::login);
}
