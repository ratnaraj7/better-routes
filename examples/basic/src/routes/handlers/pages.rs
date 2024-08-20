use axum::extract::State;
use axum_extra::routing::TypedPath;
use better_routes::method_helper;
use serde::Deserialize;

use crate::views::{Home as HomePage, Row};
use crate::AppState;

use super::api::{Todo, TodoWithId};

#[derive(Deserialize)]
pub struct Home;

#[method_helper(AppState)]
impl Home {
    #[get]
    #[allow(unused)]
    pub async fn index(self, State(AppState { db }): State<AppState>) -> HomePage {
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
            .collect();
        HomePage {
            rows,
            create_todo_url: Todo {}.to_uri().to_string(),
        }
    }
}
