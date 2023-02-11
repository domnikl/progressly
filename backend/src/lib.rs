pub mod models;
pub mod schema;
pub mod routes;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::{request::{FromRequest, Outcome}, Request, http::Status};
use std::{env, str::FromStr};
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}


#[derive(Debug)]
pub enum AuthorizationError {
    HeaderMissing,
    MissingBearer,
    InvalidToken
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorized {
    type Error = AuthorizationError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res = header_bearer_validation(req);

        match res {
            Err(e) => {
                match e {
                    AuthorizationError::HeaderMissing => Outcome::Failure((Status::Unauthorized, e)),
                    AuthorizationError::MissingBearer => Outcome::Failure((Status::Unauthorized, e)),
                    AuthorizationError::InvalidToken => Outcome::Failure((Status::Forbidden, e)),
                }
            }
            Ok(authorized) => Outcome::Success(authorized)
        }
    }
}

fn header_bearer_validation(req: &Request<'_>) -> Result<Authorized, AuthorizationError> {
    let authorization = req.headers().get_one("Authorization");
    let header = match authorization {
        None => return Err(AuthorizationError::HeaderMissing),
        Some(header) => header,
    };

    let contents = header.to_string();

    let token = match contents.starts_with("Bearer ") {
        true => contents.strip_prefix("Bearer ").unwrap_or(""),
        false => return Err(AuthorizationError::MissingBearer),
    };

    token_validation(token)
}

fn token_validation(token: &str) -> Result<Authorized, AuthorizationError> {
    let uuid = match Uuid::from_str(token) {
        Ok(uuid) => uuid,
        Err(_) => return Err(AuthorizationError::InvalidToken),
    };

     // TODO: verify token is valid and not just use user_id!
    return Ok(Authorized { user_id: uuid })
}

pub struct Authorized {
    pub user_id: Uuid,
}
