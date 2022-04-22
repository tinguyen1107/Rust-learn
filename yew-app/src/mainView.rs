use yew::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/home")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <todos::TodosComponent />
            }    
        }
        Route::NotFound => {
            html! {
                <>
                    <h1>{"Not found 404"}</h1>
                    <Link<Route> to={Route::Home}>{"Back to home page."}</Link<Route>>
                </>
            }
        }
    }
}

#[path="./Todos/todos.rs"]
mod todos;

pub enum MainViewMessage {
    
}

#[function_component(MainView)]
pub fn main_view() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    } 
}
