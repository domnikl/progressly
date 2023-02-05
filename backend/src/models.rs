use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::projects;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub color: String
}
