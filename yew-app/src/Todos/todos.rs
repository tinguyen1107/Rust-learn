use web_sys::*;
use weblog::*;
use yew::*;

#[path="../pokemon.rs"]
mod pokemon;
#[path="./todoView.rs"]
mod todo_view;
#[path="./models.rs"]
mod models;

use models::*;
use todo_view::*;

#[derive(PartialEq)]
pub struct TodosComponent {
    todos: Vec<Todo>,
    input: String,
}

impl TodosComponent {
    pub fn default() -> Self {
        Self {
            todos: Vec::new(),
            input: String::from("")
        }
    }
}

impl Component for TodosComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                let todo = Todo {
                    id: self.todos.len(),
                    title: self.input.clone(),
                    is_done: false,
                    description: None,
                    priority: Priority::Low
                };
                self.todos.push(todo);
                self.input = "".to_string();
                true
            }
            Msg::Update(v) => {
                self.input = v;
                true
            }
            Msg::ChangeCheck(id, val) => {
                self.todos[id].is_done = val;
                true
            }
            Msg::Remove(i) => {
                console::log_1(&i.into());
                if i < self.todos.len() {
                    self.todos.remove(i);
                    true
                } else {
                    false
                }
            }
            Msg::RemoveSellected => {
                self.todos = self.todos.clone().into_iter().filter(|todo| !todo.is_done).collect();
                true
            }
            Msg::RemoveAll => {
                self.todos.clear(); 
                true
            } 
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_: MouseEvent| {
            Msg::Add
        });
        
        let oninput = ctx.link().callback(|e: InputEvent| {     
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            Msg::Update(value)
        });
        
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Some(Msg::Add)
            } else {
                None
            }
        });

        let filter_click = ctx.link().callback(|_| {
            Msg::Remove(0)
        });

        let remove_sellected_click = ctx.link().callback(|_| {
            Msg::RemoveSellected
        });

        let remove_all_click = ctx.link().callback(|_| {
            Msg::RemoveAll
        });

        html! {
            <div>
                <div class="container">
                    <h2>{"Todo List"}</h2>
                    <p>{"Nums of task: "}{self.todos.len()}</p>
                    <div class="list_todos_view">
                        {
                            self.todos.clone().into_iter().map(|content| {
                                let content_outer = content.clone(); 
                                let content_change = ctx.link().callback(move |val: bool| {
                                    Msg::ChangeCheck(content.clone().id, val)
                                });
                                let props = TodoViewProps
                                {
                                    todo: content_outer, 
                                    onchange: content_change.clone()
                                };
                                html! {
                                    <TodoView ..props />
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="input_container">
                        <input type="text"
                            placeholder="Please input new task.."
                            {oninput}
                            {onkeypress}
                            value={self.input.clone()}/>
                        <button {onclick}>{"Add"}</button>
                    </div>
                    <div class="input_container">
                        <button onclick={filter_click}>{"Filter"}</button>
                        <button onclick={remove_sellected_click}>{"Remove done"}</button>
                        <button onclick={remove_all_click}>{"Remove All"}</button> 
                    </div>
                </div>
                <div class="container">
                    <pokemon::Icon />
                </div>
            </div>
        }
    }
}
