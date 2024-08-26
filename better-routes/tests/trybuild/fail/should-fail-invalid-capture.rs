use axum_extra::routing::RouterExt;
use better_routes::routes;
use serde::Deserialize;
#[derive(Deserialize)]
struct Foo;
async fn get(_: Foo) {}
routes! {
    name => AllRoutes,
    "/:id" => Foo {
        get => get
    }
}
fn main() {}
