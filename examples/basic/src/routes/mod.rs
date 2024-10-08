use axum::extract::rejection::PathRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::routing::RouterExt;
use better_routes::routes;

use crate::AppState;

use self::handlers::api::{create_todo, delete_todo, update_todo, Todo, TodoWithId};
use self::handlers::pages::{index, Home};

mod handlers;

pub struct GlobalRejection;

impl From<PathRejection> for GlobalRejection {
    fn from(_: PathRejection) -> Self {
        Self
    }
}

impl IntoResponse for GlobalRejection {
    fn into_response(self) -> Response {
        // Respond with a 404 Not Found status code for global rejections
        StatusCode::NOT_FOUND.into_response()
    }
}

routes! {
    name => pub AllRoutes,
    state => AppState,
    rejection => GlobalRejection,
    "/" => Home{
        get => index
    },
    "/api/todo" => Todo {
        post => create_todo
    },
    "/api/todo/:id" => TodoWithId {
        put => update_todo,
        delete => delete_todo,
    },
}
