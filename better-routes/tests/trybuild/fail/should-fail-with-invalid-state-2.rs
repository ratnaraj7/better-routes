use axum::extract::State;
use axum_extra::routing::RouterExt;
use better_routes::routes;
use serde::Deserialize;
#[derive(Clone)]
struct BarState;
#[derive(Clone)]
struct FooState;
#[derive(Deserialize)]
struct Foo {
    id: String,
}
async fn get(_: Foo, _: State<FooState>) {}
routes! {
    name => AllRoutes,
    state => BarState,
    "/:id" => Foo {
        get => get
    }
}
fn main() {}
