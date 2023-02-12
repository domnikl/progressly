use serde::{Deserialize};
use gloo::utils::window;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub color: String,
}

fn get_base_url() -> String {
    let location = window().location();
    
    format!(
        "{}//{}",
        location.protocol().expect("Error getting protocol"),
        location.host().expect("Error getting host"),
    )
}

pub async fn get_projects() -> Result<Vec<Project>, reqwest::Error> {
    let client = reqwest::Client::new();

    let resp = client.get(format!("{}/api/projects", get_base_url()))
        .bearer_auth("c79f9e52-86eb-4de6-be78-a7a097b8f516")
        .send()
        .await;
    
    match resp {
        Err(e) => return Err(e),
        Ok(projects) => projects.json::<Vec<Project>>().await
    }
}

/*pub async fn start_tracking() -> Result<StartTracking, reqwest::Error> {
    let client = reqwest::Client::new();

    // TODO: post body!
    let resp = client.post(format!("{}/api/trackings", get_base_url()))
        .bearer_auth("c79f9e52-86eb-4de6-be78-a7a097b8f516")
        .send()
        .await;
}*/