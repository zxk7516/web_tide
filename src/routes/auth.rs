use crate::APool;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::dump;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header};

use crate::error::{
    customer_error,
    ApiError::{self, *},
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
struct UserInfo<'a> {
    pub id: u64,
    pub username: &'a str,
    #[serde(skip)]
    pub password: &'a str,
}

#[post("/reg")]
pub async fn reg(pool: web::Data<APool>, reg: web::Json<Reg>) -> Result<HttpResponse, ApiError> {
    let row: sqlx::Result<(u64,)> = sqlx::query_as(r#"select id from users where username=?"#)
        .bind(&reg.username)
        .fetch_one(&**pool)
        .await;
    match row {
        Ok(_e) => Err(customer_error(40002, "user already exists")),
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
        _e => {
            let _ = dump!(_e);
            Err(UnknowError)
        }
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<APool>,
    login: web::Json<Reg>,
) -> Result<HttpResponse, ApiError> {
    let row: SqlGetUserByName =
        sqlx::query_as(r#"select id,username,password from users where username=?"#)
            .bind(&login.username)
            .fetch_one(&**pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => customer_error(40004, "username note exists"),
                _e => {
                    let _ = dump!(_e);
                    UnknowError
                }
            })?;
    if bcrypt::verify(&login.password, &row.password).map_err(|_e| {
        let _ = dump!(_e);
        UnknowError
    })? {
        let key = EncodingKey::from_secret("123".as_bytes());
        let alg = Algorithm::HS256;
        let headers = Header::new(alg);
        let token = encode(&headers, &row, &key).map_err(|_e| {
            let _ = dump!(_e);
            customer_error(50001, "jwt error")
        })?;
        Ok(HttpResponse::Created().json(&token))
    } else {
        Err(customer_error(1, "login failed"))
    }
}




