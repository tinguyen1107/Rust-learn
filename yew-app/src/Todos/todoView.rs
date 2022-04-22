use yew::*;
use web_sys::*;

#[path = "./models.rs"]
pub mod models;
use super::models::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TodoViewProps {
    pub todo: Todo,
    pub onchange: Callback<bool>
}

#[function_component(TodoView)]
pub fn todo_view(props: &TodoViewProps) -> Html {
    let TodoViewProps {
        todo,
        ref onchange
    } = props;
    let oninput = onchange.reform(|e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.checked()
    });
    html! {
        <div class="todoview_container" key={todo.id}>
            <input type="checkbox"
                {oninput}
                checked={todo.is_done}
                />
            <label>{todo.title.clone()}</label>
        </div>      
    } 
}
