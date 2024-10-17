use serde::Deserialize;


#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String
}