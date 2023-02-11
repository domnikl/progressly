use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub color: String,
}

pub async fn get_projects() -> Result<Vec<Project>, reqwest::Error> {
    let client = reqwest::Client::new();

    let resp = client.get("/api/projects")
        .bearer_auth("c79f9e52-86eb-4de6-be78-a7a097b8f516")
        .send()
        .await;
    
    match resp {
        Err(e) => return Err(e),
        Ok(projects) => projects.json::<Vec<Project>>().await
    }
}
