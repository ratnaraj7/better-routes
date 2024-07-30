use axum::response::{IntoResponse, Response};
use better_routes::routes;

#[derive(Default)]
struct GlobalRejection;

impl IntoResponse for GlobalRejection {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Clone)]
struct State {}

routes! {
    State => State,
    Rejection => GlobalRejection,
}

fn main() {}
