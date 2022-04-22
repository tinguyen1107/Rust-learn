use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;
use serde_json::Value;

#[path ="./views/NavigationBar.rs"]
mod navigation_bar;
use navigation_bar::{
    NavigationBar,
    Route
};

fn main() {
    yew::start_app::<App>();
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::AllProjects => html! {<AllProjects/>},
        Route::StartANewProject => html! {<StartANewProject/>},
        Route::MyProjects => html! {<MyProjects/>},
        Route::MoreInformation => html! {<MoreInformation/>},
        Route::About => html! {<About/>},
        Route::NotFound => {
            html! {
                <>
                    <h1>{"Not found 404"}</h1>
                    <Link<Route> to={Route::AllProjects}>{"Back to home page."}</Link<Route>>
                </>
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct Project {
    title: String,
    author: String,
    max_expectation: f64,
    description: String,
    current: f64
}

impl Project {
    fn new(    
        title: String,
        author: String,
        max_expectation: f64,
        description: String,
        ) -> Self {
        Self {
            title, author, max_expectation, description,
            current: 0.0
        }
    }
}

#[derive(Properties, PartialEq)]
struct ProjectItemProps {
    props: Project
}

#[function_component(ProjectItem)]
fn project_item(props: &ProjectItemProps) -> html {
    let Project{
        title,
        author,
        max_expectation,
        description,
        current
    } = props.props.clone();
    html! {
        <h1>{title}</h1> 
    }
}

#[derive(Clone)]
struct AllProjectsData {
    cur: String,
    next: String,
    prev: String,
    pokes: Vec<Poke>
}

impl AllProjectsData {
    fn new(url: String) -> Self {
        Self {
            cur: url,
            next: "".to_string(),
            prev: "".to_string(),
            pokes: Vec::<Poke>::new()
        }
    }
}

#[function_component(AllProjects)]
fn all_projects() -> html {
    #[warn(clippy::vec_init_then_push)]
    let mut projects: Vec<Project> = Vec::new();
    projects.push(Project::new(
            String::from("Organization's Finance Control"), 
            String::from("Tinguyen"), 
            9486.0, 
            String::from("We want to provide coverage of violations of human rights in Russia. We are creating a network of correspondents in Russia's regions informing the editorial board about arrests, court decisions, cases of police brutality and tortures etc. That will allow us to promptly create content for project's social media (Telegram, Twitter, Instagram). These materials may be used by media and organizations defending human rights. Thanks to that more people will know about violations of human rights happening in Russia.\nWe also plan to make public campaigns aimed to change policies violating civil and political rights. That part of work will include creating public letters and petitions, drawing attention to the subject and providing support to civic initiatives.\nAnother set of activities is legal help and crowdfunding for these who are persecuted cause of political reasons. Our organization's lawyer will provide primary legal help and we will involve other human rights organizations and outsource lawyers. We'll raise funds for persecuted for political reasons with the help of online platforms. We're also planning to organize at least three offline fundraising events."
    )));
    
    let content_data = use_mut_ref(|| AllProjectsData::new("https://pokeapi.co/api/v2/pokemon/".to_string()));
    let content_state = use_state_eq(|| Vec::<Poke>::new());
    let content = content_state.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        let content_state = content_state.clone();
        let content_data = content_data.clone();
        wasm_bindgen_futures:: spawn_local(async move {
            if content_data.borrow().cur.as_str() == "" { return; }
            let res = reqwest::get(content_data.borrow().cur.as_str())
                .await
                .unwrap();
            let text = res.text().await.unwrap();
            let val: Value = serde_json::from_str(&text).unwrap();
            let data: Vec<Value> = val["results"]
                .as_array()
                .unwrap()
                .to_vec();
            let (next_url, prev_url) = (val["next"].as_str().unwrap_or(""), val["previous"].as_str().unwrap_or(""));
            content_data.borrow_mut().cur = next_url.to_string();
            content_data.borrow_mut().prev = prev_url.to_string();
            console::log_1(&format!("DEBUG --- {:?} {:?}", next_url, prev_url).into());
            let pokes = data.iter().map(|x| {
                let (name, url) = (
                    x["name"].as_str().unwrap(),
                    x["url"].as_str().unwrap()
                );
                Poke {
                    name: name.to_string(),
                    url: url.to_string()
                }
            })
            .collect::<Vec<Poke>>();
            for poke in pokes { 
                let res = reqwest::get(poke.url)
                    .await
                    .unwrap();
                let text = res.text().await.unwrap();
                let val: Value = serde_json::from_str(&text).unwrap();
                let img_url = val["sprites"]["front_default"].as_str().unwrap_or("");
                content_data.borrow_mut().pokes.push(Poke{name: poke.name, url: img_url.to_string()});
            }
            content_state.set(content_data.borrow().clone().pokes);
            // console::log_1(&format!("read as vec: {:?}", content_state.clone()).into());
        });
    });

    if content.clone().len() == 0 {
        onclick.emit(MouseEvent::new("").unwrap());
    }
    
    html! {
        <>
            <NavigationBar/>
            <div class="body">
                <div class="left-container">
                    <h1>{"test"}</h1>
                    <h2>{"Number of pokemon: "}{(*content).len()}</h2>
                </div>
                <div class="mid-container">
                    <div class="flex-container" style={"flex-direction: column; align-items: center; margin: 20px; padding: 20px;"}>
                        <h1>{"All Projects"}</h1>
                        <span>{"Sort: "}</span>
                        <select>
                            <option value="a">{"Newest"}</option>
                            <option value="a">{"Oldest"}</option>
                            <option value="a">{"Max expectation increase"}</option>
                            <option value="a">{"Max expectation decrease"}</option>
                            <option value="a">{"A -> Z"}</option>
                            <option value="a">{"Z -> A"}</option>
                            <option value="a">{"Reliability decrease"}</option>
                            <option value="a">{"Available increase"}</option>
                            <option value="a">{"Available decrease"}</option>
                        </select>
                        {
                            projects.into_iter().map(|proj| {
                                html! {
                                    <ProjectItem props={proj}/>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="poke-container">
                        <div class="poke-header__container flex-container">
                            <button {onclick}>{"Load more Pokemons"}</button>
                            <input type="text" id="search" placeholder="Search..."/>    
                            <button >{"Search"}</button>
                        </div>
                        <hr class="solid"/>
                        <div class="poke-list__container"> 
                            {
                                (*content).clone().iter().map(|poke| {
                                    html! { <PokeView props={poke.clone()}/> }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                </div>
                <div class="right-container">
                    <h1>{"test"}</h1>
                </div>
            </div>
        </>
    }
}

// Props for Poke View
#[derive(Properties, PartialEq)]
struct PokeViewProps {
    props: Poke
}

/*
 *  Poke View
 */
#[function_component(PokeView)]
fn poke_view(props: &PokeViewProps) -> Html {
    let Poke {name, url} = props.props.clone();
    html! {
        <div class="poke-view__container flex-container">
            <img src={url.clone()}/>
            <h1>{name.to_uppercase()}</h1>
        </div>
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Poke {
    name: String,
    url: String
}

#[function_component(StartANewProject)]
fn start_a_new_project() -> Html {
    html! { 
        <>
            <NavigationBar/>
            <h1>{"Start A New Project"}</h1>
        </>
    }
}

#[function_component(MyProjects)]
fn my_projects() -> Html {
    html! { 
        <>
            <NavigationBar/>
            <h1>{"My Projects"}</h1>
        </>
    }
}

#[function_component(MoreInformation)]
fn more_infomation() -> Html {
    html! { 
        <>
            <NavigationBar/>
            <h1>{"More Information"}</h1>
        </>
    }
}

#[function_component(About)]
fn about() -> Html {
    html! { 
        <>
            <NavigationBar/>
            <h1>{"About"}</h1>
        </>
    }
}
