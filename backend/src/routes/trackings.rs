use rocket::{post, patch, serde::json::Json, time::OffsetDateTime, response::status::NotFound};
use serde::Deserialize;
use uuid::Uuid;

use crate::{establish_connection, models::trackings::Tracking, Authorized};

#[derive(Debug, Deserialize)]
pub struct NewTracking {
    pub project_id: Uuid,
    pub task_id: Option<Uuid>,
    #[serde(with = "time::serde::rfc3339")]
    pub start: OffsetDateTime,
}

#[post("/trackings", data = "<new_tracking>", format = "json")]
pub fn start_tracking(authorized: Authorized, new_tracking: Json<NewTracking>) -> Json<Tracking> {
    let connection = &mut establish_connection();
    let tracking = Tracking {
        id: Uuid::new_v4(),
        user_id: authorized.user_id,
        project_id: new_tracking.project_id,
        task_id: new_tracking.task_id,
        start: new_tracking.start,
        end: None,
    };

    tracking.create(connection);

    Json(tracking)
}

#[derive(Debug, Deserialize)]
pub struct PatchTracking {
    #[serde(with = "time::serde::rfc3339")]
    pub end: OffsetDateTime,
}

#[patch("/trackings", data = "<patch_tracking>", format = "json")]
pub fn end_tracking(authorized: Authorized, patch_tracking: Json<PatchTracking>) -> Result<Json<Tracking>, NotFound<String>> {
    let connection = &mut establish_connection();
    let mut tracking = match Tracking::find_active_by_user_id(authorized.user_id, connection) {
        None => return Err(NotFound("Active tracking not found".to_string())),
        Some(t) => t
    };

    tracking.end(patch_tracking.end, connection);

    Ok(Json(tracking))
}
