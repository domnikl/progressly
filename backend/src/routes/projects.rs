use rocket::{post, serde::json::Json, get};
use serde::Deserialize;
use uuid::Uuid;

use crate::{establish_connection, models::projects::Project, Authorized};

#[derive(Debug, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub color: String,
}

#[post("/projects", data = "<new_project>", format = "json")]
pub fn post_project(authorized: Authorized, new_project: Json<NewProject>) -> Json<Project> {
    let project = Project {
        id: Uuid::new_v4(),
        name: new_project.name.to_string(),
        user_id: authorized.user_id,
        color: new_project.color.to_string()
    };

    project.save(&mut establish_connection());

    Json(project)
}

#[get("/projects", format = "json")]
pub fn get_projects(authorized: Authorized) -> Json<Vec<Project>> {
    let connection = &mut establish_connection();
    let results = Project::find_by_user_id(authorized.user_id, connection);

    Json(results)
}
