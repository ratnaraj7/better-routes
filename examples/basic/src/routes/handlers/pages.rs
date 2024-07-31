use axum::extract::State;
use axum_extra::routing::{RouterExt, TypedPath};
use better_routes::method_helper;

use crate::routes::{Home, Todo, TodoWithId};
use crate::views::{HomeView, Row};
use crate::AppState;

#[method_helper(AppState)]
impl Home {
    #[get]
    async fn index(self, State(AppState { db }): State<AppState>) -> HomeView {
        let rows = db
            .lock()
            .await
            .iter()
            .map(|(id, (name, status))| Row {
                id: id.to_owned(),
                name: name.to_owned(),
                status: status.to_owned(),
                delete_todo_url: TodoWithId { id: *id }.to_uri().to_string(),
                update_todo_url: TodoWithId { id: *id }.to_uri().to_string(),
            })
            .collect();
        HomeView {
            rows,
            create_todo_url: Todo {}.to_uri().to_string(),
        }
    }
}
