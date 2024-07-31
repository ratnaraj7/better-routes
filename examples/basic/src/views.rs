use askama_axum::Template;

use crate::Status;

#[derive(Template)]
#[template(path = "table/row.html")]
pub struct Row {
    pub id: usize,
    pub name: String,
    pub status: Status,
    pub delete_todo_url: String,
    pub update_todo_url: String,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeView {
    pub rows: Vec<Row>,
    pub create_todo_url: String,
}
