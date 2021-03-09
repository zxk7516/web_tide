use crate::APool;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::error::{
    ApiError::{self, *},
    Msg,
};

#[derive(Deserialize)]
pub struct Reg {
    pub username: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
struct SqlGetUserByName {
    pub id: u64,
    pub username: String,
    #[serde(skip)]
    pub password: String,
}
#[derive(serde::Serialize)]
struct UserInfo<'a, 'b> {
    pub id: u64,
    pub username: &'a str,
    #[serde(skip)]
    pub password: &'b str,
}

#[post("/reg")]
pub async fn reg(pool: web::Data<APool>, reg: web::Json<Reg>) -> Result<HttpResponse, ApiError> {
    let row: sqlx::Result<(i33,)> = sqlx::query_as(r#"select id from users where username=?"#)
        .bind(&reg.username)
        .fetch_one(&**pool)
        .await;
    match row {
        Ok(_e) => Err(CustomerError(Msg::new(40002, "user already exists"))),
        Err(sqlx::Error::RowNotFound) => {
            let password = bcrypt::hash(&reg.password, 4)?;
            let id = sqlx::query(r#"insert into users (username,password) values(?,?)"#)
                .bind(&reg.username)
                .bind(&password)
                .execute(&**pool)
                .await?
                .last_insert_id();
            let u = UserInfo {
                id,
                username: &reg.username,
                password: &password,
            };
            Ok(HttpResponse::Created().json(&u))
        }
        _ => Err(UnknowError),
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<APool>,
    login: web::Json<Reg>,
) -> Result<HttpResponse, ApiError> {
    let row: SqlGetUserByName =
        sqlx::query_as(r#"select id,username,paddword from users where username=?"#)
            .bind(&login.username)
            .fetch_one(&**pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => CustomerError(Msg::new(40004, "item note exists")),
                _ => UnknowError,
            })?;
    if bcrypt::verify(&login.password, &row.password).map_err(|_| UnknowError)? {
    } else {
    }

    todo!()
}
