#[path ="./mainView.rs"]
mod main_view;

fn main() {
    yew::start_app::<main_view::MainView>();
}
