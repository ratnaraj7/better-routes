use axum_extra::routing::RouterExt;
use better_routes::{method_helper, routes};
use serde::Deserialize;
#[derive(Deserialize)]
struct Foo;
#[method_helper]
impl Foo {}
routes! {
    name => FooPath,
    "/:id" => Foo
}
fn main() {}
