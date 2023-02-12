use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::Project;

#[function_component]
pub fn Projects() -> Html {
    let projects: UseStateHandle<Vec<Project>> = use_state_eq(|| vec![]);

    {
        // make copies of the state
        let projects = projects.clone();

        use_effect_with_deps(move |_| {
            spawn_local(async move {
                match crate::api::get_projects().await {
                    Err(_e) => {}, // TODO: error handling: show a snackbar
                    Ok(p) => projects.set(p)
                }
            })
        }, ());
    }

    html!(
        <div>
            <h1>{"projects"}</h1>

            <div class="projects-list">
                {
                    projects.iter().map(|project| {
                        html!{
                            <div class="mdl-card mdl-shadow--2dp">
                                <div class="mdl-card__title">
                                    <h2 class="mdl-card__title-text">{ project.clone().name }</h2>
                                </div>
                                //<div class="mdl-card__supporting-text">
                                    
                                //</div>
                                <div class="mdl-card__actions mdl-card--border">
                                    <a class="mdl-button mdl-button--colored mdl-js-button mdl-js-ripple-effect">
                                        {"start"}
                                    </a>
                                </div>

                                <div class="mdl-card__menu">
                                    <button class="mdl-button mdl-button--icon mdl-js-button mdl-js-ripple-effect">
                                        <i class="material-icons">{"play_arrow"}</i>
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    )
}
