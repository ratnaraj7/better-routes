pub use better_routes_macros::method_helper;
pub use better_routes_macros::routes;

pub trait MethodHandler<S = ()> {
    fn router() -> axum::Router<S>;
}
