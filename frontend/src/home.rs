use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Home() -> Html {
    html!(
        <div>
            <h1>{"progressly"}</h1>

            <p>
                {"Go to "}
                <Link<Route> to={Route::Projects}>{"projects"}</Link<Route>>
                {"."}
            </p>
        </div>
    )
}
