use axum_extra::routing::RouterExt;
use better_routes::routes;
use serde::Deserialize;
#[derive(Deserialize)]
struct Home;
#[derive(Deserialize)]
struct About;
async fn about(_: About) {}
routes! {
    name => AllRoutes,
    "/" => Home {
        get => about,
    },
    "/about" => About {
        get => about
    },
}
fn main() {}
