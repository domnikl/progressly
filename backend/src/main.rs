use rocket::fs::{FileServer, relative};

use progressly::routes::{projects::*, trackings::*};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_projects, post_project, start_tracking, end_tracking])
}
