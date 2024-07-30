use axum::response::{IntoResponse, Response};
use axum_extra::routing::RouterExt;
use better_routes::{method_helper, routes};

#[derive(Default)]
struct GlobalRejection;

impl IntoResponse for GlobalRejection {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Default)]
struct FooRejection;

impl IntoResponse for FooRejection {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Clone)]
struct State {}

routes! {
    State => State,
    Rejection => GlobalRejection,
    "/foo" => struct Foo; => FooRejection,
    "/bar" => struct Bar;,
}

#[method_helper(State)]
impl Foo {}

#[method_helper(State)]
impl Bar {
    #[get]
    async fn bar(self) {}
}

fn main() {}
