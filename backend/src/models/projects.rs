use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub color: String
}

impl Project {
    pub fn find_by_user_id(uid: Uuid, connection: &mut PgConnection) -> Vec<Project> {
        use crate::schema::projects::dsl::*;

        let results = projects
            .filter(user_id.eq(uid))
            .order_by(name)
            .limit(100)
            .load::<Project>(connection)
            .expect("Error loading posts");

        return results;
    }

    pub fn save(&self, connection: &mut PgConnection) {
        use crate::schema::projects::dsl::*;

         diesel::insert_into(projects)
            .values(self)
            .execute(connection)
            .expect("Error saving new post");
    }
}
