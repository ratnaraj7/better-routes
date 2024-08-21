use axum::extract::State;
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
    async fn handle_get(self, State(_): State<FooState>) {}
}
#[derive(Clone)]
struct BarState;
routes! {
    name => FooPath,
    state => BarState,
    "/:id" => Foo
}
fn main() {}
