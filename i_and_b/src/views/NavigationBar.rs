use yew_router::prelude::*;
use yew::prelude::*;
use web_sys::HtmlElement;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/all_projects")]
    AllProjects,
    #[at("/start_a_new_project")]
    StartANewProject,
    #[at("/my_projects")]
    MyProjects,
    #[at("/more_infomation")]
    MoreInformation,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html { 
    let navigator = use_history().unwrap();
    let onclick = Callback::from(move |e: MouseEvent| {
        let button_id = e.target_unchecked_into::<HtmlElement>().id();
        let route = match button_id.as_str() {
            "all_projects" => Route::AllProjects,
            "start_a_new_project" => Route::StartANewProject,
            "my_projects" => Route::MyProjects,
            "more_infomation" => Route::MoreInformation,
            "about" => Route::About,
            _ => Route::NotFound
        };
        navigator.push(route);
    });
    html! {
        <div class="navbar__container">
            <img src="img/logo.svg" alt="Italian Trulli" />
            <div class="navbar__fn-container">
                <button id={"all_projects"} onclick={onclick.clone()}>{"All Projects"}</button>
                <button id={"start_a_new_project"} onclick={onclick.clone()}>{"Start a new Projects"}</button>
                <button id={"my_projects"} onclick={onclick.clone()}>{"My Projects"}</button>
                <button id={"more_infomation"} onclick={onclick.clone()}>{"More Information"}</button>
                <button id={"about"} onclick={onclick.clone()}>{"About I&B"}</button>
            </div>
        </div>
    }
}
