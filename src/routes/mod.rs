use crate::error::ApiError;
use actix_web::{get, web,  HttpResponse};
mod auth;

#[get("/ping")]
async fn hello() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("pong"))
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(auth::reg);
}
