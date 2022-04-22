pub enum Msg {
    Add,
    Update(String),
    ChangeCheck(usize, bool),
    Remove(usize),
    RemoveSellected,
    RemoveAll,
}

#[derive(PartialEq, Clone)]
pub enum Priority {
    Urgent,
    Hight,
    Low
}

#[derive(PartialEq, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub is_done: bool,
    pub description: Option<String>,
    pub priority: Priority,
}
