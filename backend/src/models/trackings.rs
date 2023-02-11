use diesel::{PgConnection, Queryable, Insertable, prelude::*};
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Queryable, Identifiable, Insertable, Serialize, Clone, Copy)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(table_name = crate::schema::trackings)]
pub struct Tracking {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub task_id: Option<Uuid>,
    #[serde(with = "time::serde::rfc3339")]
    pub start: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub end: Option<OffsetDateTime>
}

impl Tracking {
    pub fn find_active_by_user_id(uid: Uuid, connection: &mut PgConnection) -> Option<Tracking> {
        use crate::schema::trackings::dsl::*;

        trackings
            .filter(user_id.eq(uid))
            .filter(end.is_null())
            .first::<Tracking>(connection)
            .optional()
            .unwrap()
    }

    pub fn create(&self, connection: &mut PgConnection) {
        use crate::schema::trackings::dsl::*;

        // TODO: check that there is no tracking in progress right now

        diesel::insert_into(trackings)
            .values(self)
            .execute(connection)
            .expect("Error saving tracking");
    }

    pub fn end(&mut self, e: OffsetDateTime, connection: &mut PgConnection) {
        use crate::schema::trackings::dsl::*;

        // set field in struct
        self.end = Some(e);
        let written = &*self;

        diesel::update(written)
            .set(end.eq(self.end))
            .execute(connection)
            .expect("Error ending tracking");
    }
}
