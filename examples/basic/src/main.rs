use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use tokio::sync::Mutex;

use self::routes::AllRoutes;
use self::views::Status;

mod routes;
mod views;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<HashMap<usize, (String, Status)>>>,
}

#[tokio::main]
async fn main() {
    let r = AllRoutes::routes();
    let r = r.layer(from_fn(validate_req));
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");
    let state = AppState {
        db: Arc::new(Mutex::new(HashMap::new())),
    };

    axum::serve(tcp_listener, r.with_state(state))
        .await
        .expect("Failed to start server");
}

async fn validate_req(req: Request, next: Next) -> Response {
    // Reject non-HTMX requests for api routes
    if req.uri().path().starts_with("/api/") && !req.headers().contains_key("HX-Request") {
        return StatusCode::BAD_REQUEST.into_response();
    }
    next.run(req).await
}
