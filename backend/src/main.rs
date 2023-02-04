use rocket::{serde::{Serialize, Deserialize, json::Json}};
use rocket::fs::{FileServer, relative};

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
pub struct Project {
    id: String,
    name: String,
}


#[get("/project", format = "json")]
fn get_project() -> Json<Project> {
    Json(Project { id: "abc".to_string(), name: "Reading".to_string() })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_project])
}
