pub mod home;
pub mod projects;
pub mod api;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <home::Home /> },
        MainRoute::Projects => html! { <projects::Projects />},
        MainRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub children: Children,
}

#[function_component(Navigation)]
fn navigation(props: &NavigationProps) -> Html {
html! {
        <div class="demo-layout-waterfall mdl-layout mdl-js-layout">
        <header class="mdl-layout__header mdl-layout__header--waterfall">
            <div class="mdl-layout__header-row">
            <span class="mdl-layout-title"><a href="/">{"progressly"}</a></span>
            <div class="mdl-layout-spacer"></div>
            <div class="mdl-textfield mdl-js-textfield mdl-textfield--expandable
                        mdl-textfield--floating-label mdl-textfield--align-right">
                <label class="mdl-button mdl-js-button mdl-button--icon" for="waterfall-exp">
                <i class="material-icons">{"search"}</i>
                </label>
                /*<div class="mdl-textfield__expandable-holder">
                    <input class="mdl-textfield__input" type="text" name="sample" id="waterfall-exp" />
                </div>*/
            </div>
            </div>
            <div class="mdl-layout__header-row">
                <div class="mdl-layout-spacer"></div>
                <nav class="mdl-navigation">
                    <Link<MainRoute> classes={classes!("mdl-navigation__link")} to={MainRoute::Projects}>{"Projects"}</Link<MainRoute>>
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
                { for props.children.iter() }
            </div>
        </main>
        <footer class="mdl-mini-footer">
            <div class="mdl-mini-footer__left-section">
                <div class="mdl-logo">{"progressly"}</div>
                <ul class="mdl-mini-footer__link-list">
                <li><a href="https://github.com/domnikl/progressly">{"GitHub"}</a></li>
                </ul>
            </div>
        </footer>
        </div>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <HashRouter>
            <Navigation>
                <Switch<MainRoute> render={switch_main} />
            </Navigation>
        </HashRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<Main>::new().render();
}
