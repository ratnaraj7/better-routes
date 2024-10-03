use axum::extract::State;
use axum::Form;
use axum_extra::routing::TypedPath;
use origami_engine::Origami;
use serde::Deserialize;
use tailwind_fuse::IntoTailwindClass;

use crate::views::{row, StatusClass};
use crate::{AppState, Status};

#[derive(Deserialize)]
pub struct CreateTodoPayload {
    name: String,
}

#[derive(Deserialize)]
pub struct Todo;

pub async fn create_todo(
    _: Todo,
    State(AppState { db }): State<AppState>,
    Form(CreateTodoPayload { name }): Form<CreateTodoPayload>,
) -> Origami {
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
    row! {
        id { @(format!("row-{}", id).as_str()) },
        status { @(StatusClass { status }.to_class().as_str()) },
        name { *name.as_str(); },
        delete_todo_url { @(TodoWithId {id}.to_uri().to_string().as_str()) },
        target { @(format!("#row-{}", id).as_str()) },
        onclick { @(format!("show_dialog('#row-{}', '{}', '{}', '{}')", id, TodoWithId { id }.to_uri(), name, status).as_str()) }
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

pub async fn update_todo(
    TodoWithId { id }: TodoWithId,
    State(AppState { db }): State<AppState>,
    Form(UpdateTodoPayload { name, status }): Form<UpdateTodoPayload>,
) -> Origami {
    *db.lock().await.get_mut(&id).unwrap() = (name.clone(), status);

    row! {
        id { @(format!("row-{}", id).as_str()) },
        status { @(StatusClass { status }.to_class().as_str()) },
        name { *name.as_str(); },
        delete_todo_url { @(TodoWithId {id}.to_uri().to_string().as_str()) },
        target { @(format!("#row-{}", id).as_str()) },
        onclick { @(format!("show_dialog('#row-{}', '{}', '{}', '{}')", id, TodoWithId { id }.to_uri(), name, status).as_str()) }
    }
}

pub async fn delete_todo(TodoWithId { id }: TodoWithId, State(AppState { db }): State<AppState>) {
    db.lock().await.remove(&id);
}
