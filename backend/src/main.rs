use std::str::FromStr;

use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use uuid::Uuid;
use diesel::prelude::*;
use progressly::{models::*, establish_connection};
use rocket::serde::json::Json;
use rocket::fs::{FileServer, relative};

#[macro_use] extern crate rocket;

pub struct Authorized {
    pub user_id: Uuid,
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

#[get("/projects", format = "json")]
fn get_projects(authorized: Authorized) -> Json<Vec<Project>> {
    use progressly::schema::projects::dsl::*;

    let connection = &mut establish_connection();
    let results = projects
        .filter(user_id.eq(authorized.user_id))
        .limit(100)
        .load::<Project>(connection)
        .expect("Error loading posts");

    Json(results)
}

#[post("/projects", data = "<new_project>", format = "json")]
fn post_project(authorized: Authorized, new_project: Json<Project>) -> Json<Project> {
    use progressly::schema::projects::dsl::*;

    let connection = &mut establish_connection();
    let project = Project {
        id: new_project.id,
        name: new_project.name.to_string(),
        user_id: authorized.user_id,
        color: new_project.color.to_string()
    };

    diesel::insert_into(projects)
        .values(&project)
        .execute(connection)
        .expect("Error saving new post");

    Json(project)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_projects, post_project])
}
