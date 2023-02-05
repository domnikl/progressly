use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub color: String
}
