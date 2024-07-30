//! Better Routes
//!
//! `better_routes` is a library for generating
//! [axum](https://github.com/tokio-rs/axum) routers in a type-safe and maintainable way.
//! It provides a set of macros to define routes and their handlers with strong type
//! guarantees, making routing easier and less error-prone.
//!
//! Key features include:
//! - **Type-safe routing**: Define routes using [TypedPath](https://docs.rs/axum-extra/latest/axum_extra/routing/trait.TypedPath.html).
//! - **Automatic handler generation**: Use the [`routes!`] macro to generate the router
//!   instance and the [`method_helper`] macro to implement the [`MethodHandler`] trait.
//! - **State management**: Integrate application state seamlessly into your routes.
//! - **Flexible rejection handling**: Apply global or route-specific rejection handling
//!   using custom rejection types.
//!
//! # Examples
//!
//! For various usage examples, check out the [examples directory](https://github.com/ratnaraj7/better-routes/tree/main/examples) in the repository.
//!
//! # Example
//!
//! ```
//! use better_routes::{routes, method_helper};
//! use axum_extra::routing::RouterExt;
//! use axum::Router;
//! use axum::http::StatusCode;
//! use axum::response::IntoResponse;
//! use axum::extract::rejection::PathRejection;
//! use axum::Json;
//!
//! // Define a custom rejection type for handling global rejections
//! #[derive(Default)]
//! struct GlobalRejection;
//!
//! impl From<PathRejection> for GlobalRejection {
//!     fn from(_: PathRejection) -> Self {
//!         Self
//!     }
//! }
//!
//! impl IntoResponse for GlobalRejection {
//!     fn into_response(self) -> axum::response::Response {
//!         // Respond with a 404 Not Found status code for global rejections
//!         StatusCode::NOT_FOUND.into_response()
//!     }
//! }
//!
//! // Define a custom rejection type for handling user-specific rejections
//! #[derive(Default)]
//! struct UserRejection;
//!
//! impl From<PathRejection> for UserRejection {
//!     fn from(_: PathRejection) -> Self {
//!         Self
//!     }
//! }
//!
//! impl IntoResponse for UserRejection {
//!     fn into_response(self) -> axum::response::Response {
//!         // Respond with a 400 Bad Request status code for user-specific rejections
//!         StatusCode::BAD_REQUEST.into_response()
//!     }
//! }
//!
//! // Define application state struct
//! #[derive(Clone)]
//! struct AppState;
//!
//! // Use the `routes!` macro to define routes and associate them with handlers and rejections
//! routes! {
//!     State => AppState, // Specify application state
//!     Rejection => GlobalRejection, // Apply global rejection handler
//!     "/product/:id" => struct Product { id: usize }, // Define route for products
//!     "/users/:id" => struct Users { id: usize } => UserRejection, // Define route for users with custom rejection
//! }
//!
//! // Implement handlers for the `Product` struct
//! #[method_helper(AppState)]
//! impl Product {
//!     #[get]
//!     async fn get(self) -> Json<String> {
//!         // Print the ID to the console
//!         println!("Product ID: {}", self.id);
//!         // Return a response with the ID
//!         Json(format!("Product ID: {}", self.id))
//!     }
//!
//!     #[post]
//!     async fn create(self) -> Json<String> {
//!         println!("Creating product with ID: {}", self.id);
//!         Json(format!("Created product with ID: {}", self.id))
//!     }
//!
//!     #[put]
//!     async fn update(self) -> Json<String> {
//!         println!("Updating product with ID: {}", self.id);
//!         Json(format!("Updated product with ID: {}", self.id))
//!     }
//!
//!     #[delete]
//!     async fn delete(self) -> Json<String> {
//!         println!("Deleting product with ID: {}", self.id);
//!         Json(format!("Deleted product with ID: {}", self.id))
//!     }
//!
//!     #[patch]
//!     async fn patch(self) -> Json<String> {
//!         println!("Patching product with ID: {}", self.id);
//!         Json(format!("Patched product with ID: {}", self.id))
//!     }
//! }
//!
//! // Implement handlers for the `Users` struct
//! #[method_helper(AppState)]
//! impl Users {
//!     #[get]
//!     async fn get(self) -> Json<String> {
//!         println!("User ID: {}", self.id);
//!         Json(format!("User ID: {}", self.id))
//!     }
//!
//!     #[post]
//!     async fn create(self) -> Json<String> {
//!         println!("Creating user with ID: {}", self.id);
//!         Json(format!("Created user with ID: {}", self.id))
//!     }
//!
//!     #[put]
//!     async fn update(self) -> Json<String> {
//!         println!("Updating user with ID: {}", self.id);
//!         Json(format!("Updated user with ID: {}", self.id))
//!     }
//!
//!     #[delete]
//!     async fn delete(self) -> Json<String> {
//!         println!("Deleting user with ID: {}", self.id);
//!         Json(format!("Deleted user with ID: {}", self.id))
//!     }
//!
//!     #[patch]
//!     async fn patch(self) -> Json<String> {
//!         println!("Patching user with ID: {}", self.id);
//!         Json(format!("Patched user with ID: {}", self.id))
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a Router instance using the `router()`
//!     // method generated by the [`routes!`] macro
//!     let r: Router<AppState> = router();
//! }
//! ```
//!
//! For more information, see the [documentation](https://docs.rs/better_routes).
//! Contributions and feedback are welcome on [GitHub](https://github.com/ratnaraj7/better-routes).

/// A macro for implementing the [`MethodHandler`] trait
/// with the specified methods and state.
///
/// # Example
/// ```
/// use better_routes::{routes, method_helper};
/// use axum_extra::routing::RouterExt;
/// use axum::Router;
///
/// #[derive(Clone)]
/// struct AppState;
///
/// routes! {
///     State => AppState,
///     "/" => struct Home;
/// }
///
/// #[method_helper(AppState)]
/// impl Home {
///     #[get]
///     async fn index(self) {}
///
///     #[post]
///     async fn create(self) {}
///
///     #[put]
///     async fn update(self) {}
///
///     #[delete]
///     async fn delete(self) {}
///
///     #[patch]
///     async fn patch(self) {}
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // Create a Router instance using the `router()`
///     // method generated by the `routes!` macro
///     let r: Router<AppState> = router();
/// }
/// ```
pub use better_routes_macros::method_helper;

/// Define routes using the [`routes!`] macro.
///
/// The [`routes!`] macro generates the `router()` method,
/// which creates a `Router` instance configured with the
/// routes defined in the macro. Every typed path used in
/// the routes must implement the [`MethodHandler`] trait,
/// which is facilitated by the [`method_helper`] macro.
/// Implementing `MethodHandler` manually is not recommended.
///
/// # Example
/// ```
/// use better_routes::{routes, method_helper};
/// use axum_extra::routing::RouterExt;
/// use axum::Router;
///
/// routes! {
///     "/" => struct Home;
/// }
///
/// #[method_helper]
/// impl Home {
///     #[get]
///     async fn index(self) {}
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // Create a Router instance using the `router()`
///     // method generated by the `routes!` macro
///     let r: Router = router();
/// }
/// ```
///
/// # With State
/// ```
/// use better_routes::{routes, method_helper};
/// use axum_extra::routing::RouterExt;
/// use axum::Router;
///
/// #[derive(Clone)]
/// struct AppState;
///
/// routes! {
///     State => AppState,
///     "/" => struct Home;
/// }
///
/// #[method_helper(AppState)]
/// impl Home {
///     #[get]
///     async fn index(self) {}
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // Create a Router instance with state using the `router()`
///     // method generated by the `routes!` macro
///     let r: Router<AppState> = router();
/// }
/// ```
///
/// # With Global Rejection
/// ```
/// use better_routes::{routes, method_helper};
/// use axum_extra::routing::RouterExt;
/// use axum::http::StatusCode;
/// use axum::response::IntoResponse;
/// use axum::extract::rejection::PathRejection;
/// use axum::response::Response;
/// use axum::Router;
///
/// #[derive(Default)]
/// struct GlobalRejection;
///
/// impl From<PathRejection> for GlobalRejection {
///     fn from(_: PathRejection) -> Self {
///         Self
///     }
/// }
///
/// impl IntoResponse for GlobalRejection {
///     fn into_response(self) -> Response {
///         StatusCode::NOT_FOUND.into_response()
///     }
/// }
///
/// routes! {
///     Rejection => GlobalRejection, // Applied globally if specific rejections are not defined for routes
///     "/:id" => struct Home { id: usize }
/// }
///
/// #[method_helper]
/// impl Home {
///     #[get]
///     async fn index(self) {}
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // Create a Router instance using the `router()`
///     // method generated by the `routes!` macro
///     let r: Router = router();
/// }
/// ```
///
/// # With Route-Specific Rejection
/// ```
/// use better_routes::{routes, method_helper};
/// use axum_extra::routing::RouterExt;
/// use axum::http::StatusCode;
/// use axum::response::IntoResponse;
/// use axum::extract::rejection::PathRejection;
/// use axum::response::Response;
/// use axum::Router;
///
/// #[derive(Default)]
/// struct HomeRejection;
///
/// impl From<PathRejection> for HomeRejection {
///     fn from(_: PathRejection) -> Self {
///         Self
///     }
/// }
///
/// impl IntoResponse for HomeRejection {
///     fn into_response(self) -> Response {
///         StatusCode::NOT_FOUND.into_response()
///     }
/// }
///
/// routes! {
///     "/:id" => struct Home { id: usize } => HomeRejection // Rejection applies only to this specific route
/// }
///
/// #[method_helper]
/// impl Home {
///     #[get]
///     async fn index(self) {}
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // Create a Router instance using the `router()`
///     // method generated by the `routes!` macro
///     let r: Router = router();
/// }
/// ```
pub use better_routes_macros::routes;

/// The [`MethodHandler`] trait defines a method for creating a `Router` instance.
///
/// Note: Instead of implementing it directly, you should use the [`method_helper`]
/// macro to generate the necessary implementations.
pub trait MethodHandler<S = ()> {
    fn router() -> axum::Router<S>;
}
