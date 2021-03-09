extern crate serde;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer};
use sqlx::mysql::MySqlPoolOptions;

mod config;
pub(crate) mod error;
mod models;
mod routes;
pub use error::ServerResult;

type APool = sqlx::pool::Pool<sqlx::MySql>;

#[get("/")]
async fn index(_pool: web::Data<APool>) -> Result<HttpResponse, error::ApiError> {
    Ok(HttpResponse::Ok().body("normally say: it's OK."))
}

#[actix_web::main]
async fn main() -> ServerResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(6)
        .connect(&db_url)
        .await?;
    Ok(HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(index)
            .configure(routes::config)
    })
    .bind("localhost:8000")?
    .run()
    .await?)
}
