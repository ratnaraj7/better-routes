use axum::extract::State;
use axum_extra::routing::TypedPath;
use origami_engine::Origami;
use serde::Deserialize;
use tailwind_fuse::IntoTailwindClass;

use crate::views::{home, layout, row, row_head, Row, StatusClass};
use crate::AppState;

use super::api::{Todo, TodoWithId};

#[derive(Deserialize)]
pub struct Home;

pub async fn index(_: Home, State(AppState { db }): State<AppState>) -> Origami {
    let rows = db
        .lock()
        .await
        .iter()
        .map(|(id, (name, status))| Row {
            id: *id,
            name: name.to_owned(),
            status: status.to_owned(),
            delete_todo_url: TodoWithId { id: *id }.to_uri().to_string(),
            update_todo_url: TodoWithId { id: *id }.to_uri().to_string(),
        })
        .collect::<Vec<_>>();
    home!(create_todo_url { @Todo{}.to_uri().to_string().as_str() }, rows { rows })
}
