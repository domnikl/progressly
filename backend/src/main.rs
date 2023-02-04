use rocket::serde::{Serialize, Deserialize, json::Json};

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
pub struct Project {
    id: String,
    name: String,
}


#[get("/", format = "json")]
fn index() -> Json<Project> {
    Json(Project { id: "abc".to_string(), name: "Reading".to_string() })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
