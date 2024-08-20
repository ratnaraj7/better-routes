use axum::extract::State;
use axum::Form;
use axum_extra::routing::TypedPath;
use better_routes::method_helper;
use serde::Deserialize;

use crate::views::Row;
use crate::{AppState, Status};

#[derive(Deserialize)]
pub struct CreateTodoPayload {
    name: String,
}

#[derive(Deserialize)]
pub struct Todo;

#[method_helper(AppState)]
impl Todo {
    #[post]
    #[allow(unused)]
    pub async fn create_todo(
        self,
        State(AppState { db }): State<AppState>,
        Form(CreateTodoPayload { name }): Form<CreateTodoPayload>,
    ) -> Row {
        let mut id = 0;
        loop {
            if !db.lock().await.contains_key(&id) {
                break;
            }
            id += 1;
        }
        let (name, status) = db
            .lock()
            .await
            .entry(id)
            .or_insert((name, Status::Todo))
            .clone();
        Row {
            id,
            name,
            status,
            delete_todo_url: TodoWithId { id }.to_uri().to_string(),
            update_todo_url: TodoWithId { id }.to_uri().to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateTodoPayload {
    name: String,
    status: Status,
}

#[derive(Deserialize)]
pub struct TodoWithId {
    pub id: usize,
}

#[method_helper(AppState)]
impl TodoWithId {
    #[put]
    #[allow(unused)]
    pub async fn update_todo(
        self,
        State(AppState { db }): State<AppState>,
        Form(UpdateTodoPayload { name, status }): Form<UpdateTodoPayload>,
    ) -> Row {
        *db.lock().await.get_mut(&self.id).unwrap() = (name.clone(), status.clone());

        Row {
            id: self.id,
            name,
            status,
            delete_todo_url: TodoWithId { id: self.id }.to_uri().to_string(),
            update_todo_url: TodoWithId { id: self.id }.to_uri().to_string(),
        }
    }

    #[delete]
    #[allow(unused)]
    pub async fn delete_todo(self, State(AppState { db }): State<AppState>) {
        db.lock().await.remove(&self.id);
    }
}
