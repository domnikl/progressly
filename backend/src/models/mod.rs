pub mod trackings;
pub mod projects;

use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}
