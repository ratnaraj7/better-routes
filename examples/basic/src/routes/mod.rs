use axum::extract::rejection::PathRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use better_routes::routes;

use crate::AppState;

mod handlers;

struct GlobalRejection;

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
    State => AppState,
    Rejection => GlobalRejection,
    "/" => struct Home {},
    "/api/todo" => struct Todo {},
    "/api/todo/:id" => struct TodoWithId { id: usize },
}
