pub mod home;
pub mod projects;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home::Home /> },
        Route::Projects => html! { <projects::Projects />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn Router() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="demo-layout-waterfall mdl-layout mdl-js-layout">
        <header class="mdl-layout__header mdl-layout__header--waterfall">
            <div class="mdl-layout__header-row">
            <span class="mdl-layout-title">{"progressly"}</span>
            <div class="mdl-layout-spacer"></div>
            <div class="mdl-textfield mdl-js-textfield mdl-textfield--expandable
                        mdl-textfield--floating-label mdl-textfield--align-right">
                <label class="mdl-button mdl-js-button mdl-button--icon"
                    for="waterfall-exp">
                <i class="material-icons">{"search"}</i>
                </label>
                <div class="mdl-textfield__expandable-holder">
                <input class="mdl-textfield__input" type="text" name="sample" id="waterfall-exp" />
                </div>
            </div>
            </div>
            <div class="mdl-layout__header-row">
            <div class="mdl-layout-spacer"></div>
            <nav class="mdl-navigation">
                <a class="mdl-navigation__link" href="">{"Link"}</a>
                <a class="mdl-navigation__link" href="">{"Link"}</a>
                <a class="mdl-navigation__link" href="">{"Link"}</a>
                <a class="mdl-navigation__link" href="">{"Link"}</a>
            </nav>
            </div>
        </header>
        <div class="mdl-layout__drawer">
            <span class="mdl-layout-title">{"progressly"}</span>
            <nav class="mdl-navigation">
            <a class="mdl-navigation__link" href="">{"Link"}</a>
            <a class="mdl-navigation__link" href="">{"Link"}</a>
            <a class="mdl-navigation__link" href="">{"Link"}</a>
            <a class="mdl-navigation__link" href="">{"Link"}</a>
            </nav>
        </div>
        <main class="mdl-layout__content">
            <div class="page-content">
                <Router />
            </div>
        </main>
        <footer class="mdl-mini-footer">
            <div class="mdl-mini-footer__left-section">
                <div class="mdl-logo">{"progressly"}</div>
                <ul class="mdl-mini-footer__link-list">
                <li><a href="#">{"Help"}</a></li>
                <li><a href="#">{"Privacy & Terms"}</a></li>
                </ul>
            </div>
        </footer>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
