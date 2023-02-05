use uuid::uuid;
use diesel::prelude::*;
use progressly::{models::*, establish_connection};
use rocket::serde::json::Json;
use rocket::fs::{FileServer, relative};

#[macro_use] extern crate rocket;

#[get("/projects", format = "json")]
fn get_projects() -> Json<Vec<Project>> {
    use progressly::schema::projects::dsl::*;

    // TODO: remove hardcoded user id
    let x = uuid!("c79f9e52-86eb-4de6-be78-a7a097b8f516");

    let connection = &mut establish_connection();
    let results = projects
        .filter(user_id.eq(x))
        .limit(100)
        .load::<Project>(connection)
        .expect("Error loading posts");

    Json(results)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_projects])
}
