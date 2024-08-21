use axum_extra::routing::RouterExt;
use better_routes::{method_helper, routes};
use serde::Deserialize;
#[derive(Deserialize)]
struct Foo {
    id: String,
}
#[derive(Clone)]
struct FooState;
#[method_helper]
impl Foo {
    #[get]
    async fn handle_get(self) {}
}
routes! {
    name => FooPath,
    state => FooState,
    "/:id" => Foo
}
fn main() {}
