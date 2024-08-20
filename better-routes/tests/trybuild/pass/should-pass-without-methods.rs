use better_routes::method_helper;
use serde::Deserialize;
#[derive(Deserialize)]
struct Foo {
    id: String,
}
#[method_helper]
impl Foo {}
fn main() {}
