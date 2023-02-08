use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::Project;

#[function_component]
pub fn Projects() -> Html {
    let projects: UseStateHandle<Vec<Project>> = use_state_eq(|| vec![]);

    {
        let projects = projects.clone();
        use_effect(|| {
            spawn_local(async move {
                match crate::api::get_projects().await {
                    Err(_e) => {}, // TODO: error handling: show a snackbar
                    Ok(p) => projects.set(p)
                }
            })
        });
    }

    html!(
        <div>
            <h1>{"projects"}</h1>

            <div>
                {
                    projects.iter().map(|project| {
                        html!{<div key={project.clone().name}>{ format!("{}", project.name) }</div>}
                    }).collect::<Html>()
                }
            </div>

            <p>
                {"Here will be your projects"}
            </p>
        </div>
    )
}
