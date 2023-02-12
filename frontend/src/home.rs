use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute;

#[function_component]
pub fn Home() -> Html {
    html!(
        <div>
            <h1>{"progressly"}</h1>

            <p>
                {"Go to "}
                <Link<MainRoute> to={MainRoute::Projects}>{"projects"}</Link<MainRoute>>
                {"."}
            </p>
        </div>
    )
}
