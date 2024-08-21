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
//! ```rust,no_run
//! use better_routes::{routes, method_helper};
//! use axum_extra::routing::RouterExt;
//! use serde::Deserialize;
//! use axum::{
//!     Router,
//!     Json,
//!     http::StatusCode,
//!     response::IntoResponse,
//!     extract::rejection::PathRejection,
//! };
//!
//! // Define a custom rejection type for handling global rejections
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
//! // Define a struct for products
//! #[derive(Deserialize)]
//! struct Product {
//!     id: usize,
//! }
//!
//! // Define a struct for users
//! #[derive(Deserialize)]
//! struct User {
//!     id: usize,
//! }
//!
//! // Use the `routes!` macro to define routes and associate them with handlers and rejections
//! // The `name` specified here, `AllRoutes`, will be used to create a struct
//! // by the `routes!` macro. This struct implements the `routes()` method,
//! // which generates the router instance with all defined routes.
//! routes! {
//!     name => pub AllRoutes, // You can also specify visibility for the generated struct
//!     state => AppState, // Specify application state
//!     rejection => GlobalRejection, // Apply global rejection handler
//!     "/product/:id" => Product, // Define route for product
//!     "/user/:id" => User => UserRejection, // Define route for user with custom rejection
//! }
//!
//! // Implement handlers for the `Product` struct
//! #[method_helper]
//! impl Product {
//!     #[get]
//!     async fn get_product(self) -> Json<String> {
//!         // Print the ID to the console
//!         println!("Product ID: {}", self.id);
//!         // Return a response with the ID
//!         Json(format!("Product ID: {}", self.id))
//!     }
//!
//!     #[post]
//!     async fn create_product(self) -> Json<String> {
//!         println!("Creating product with ID: {}", self.id);
//!         Json(format!("Created product with ID: {}", self.id))
//!     }
//!
//!     #[put]
//!     async fn update_product(self) -> Json<String> {
//!         println!("Updating product with ID: {}", self.id);
//!         Json(format!("Updated product with ID: {}", self.id))
//!     }
//!
//!     #[delete]
//!     async fn delete_product(self) -> Json<String> {
//!         println!("Deleting product with ID: {}", self.id);
//!         Json(format!("Deleted product with ID: {}", self.id))
//!     }
//!
//!     #[patch]
//!     async fn patch_product(self) -> Json<String> {
//!         println!("Patching product with ID: {}", self.id);
//!         Json(format!("Patched product with ID: {}", self.id))
//!     }
//! }
//!
//! // Implement handlers for the `User` struct
//! #[method_helper]
//! impl User {
//!     #[get]
//!     async fn get_user(self) -> Json<String> {
//!         println!("User ID: {}", self.id);
//!         Json(format!("User ID: {}", self.id))
//!     }
//!
//!     #[post]
//!     async fn create_user(self) -> Json<String> {
//!         println!("Creating user with ID: {}", self.id);
//!         Json(format!("Created user with ID: {}", self.id))
//!     }
//!
//!     #[put]
//!     async fn update_user(self) -> Json<String> {
//!         println!("Updating user with ID: {}", self.id);
//!         Json(format!("Updated user with ID: {}", self.id))
//!     }
//!
//!     #[delete]
//!     async fn delete_user(self) -> Json<String> {
//!         println!("Deleting user with ID: {}", self.id);
//!         Json(format!("Deleted user with ID: {}", self.id))
//!     }
//!
//!     #[patch]
//!     async fn patch_user(self) -> Json<String> {
//!         println!("Patching user with ID: {}", self.id);
//!         Json(format!("Patched user with ID: {}", self.id))
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Generate the router instance using the `routes()`
//!     // method provided by the `AllRoutes` type, which is
//!     // implemented by the `routes!` macro
//!     let r: Router<AppState> = AllRoutes::routes();
//!
//!     let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
//!     let state = AppState;
//!     axum::serve(tcp_listener, r.with_state(state)).await.unwrap();
//! }
//! ```
//!
//! For more information, see the [documentation](https://docs.rs/better_routes).
//! Contributions and feedback are welcome on [GitHub](https://github.com/ratnaraj7/better-routes).

/// A macro for implementing the [`MethodHandlers`] trait.
///
/// # Example
/// ```rust
/// use better_routes::method_helper;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home;
///
/// #[method_helper]
/// impl Home {
///     #[get]
///     async fn get_(self) {}
///
///     #[post]
///     async fn create_(self) {}
///
///     #[put]
///     async fn update_(self) {}
///
///     #[delete]
///     async fn delete_(self) {}
///
///     #[patch]
///     async fn patch_(self) {}
/// }
/// ```
pub use better_routes_macros::method_helper;

/// Define routes using the [`routes!`] macro.
///
/// The [`routes!`] macro generates a struct, using the name provided, such as `AllRoutes` below,
/// that implements the `routes()` method. This method creates a
/// `Router` instance configured with the routes defined in the macro.
/// You can control the visibility of the generated struct and methods using
/// visibility modifiers.
/// Every typed path used in the routes must implement the [`MethodHandlers`] trait,
/// which is facilitated by the [`method_helper`] macro.
/// Implementing `MethodHandlers` manually is not recommended.
///
/// # Example
/// ```rust
/// use better_routes::{routes, method_helper};
/// use axum::{
///     Router,
///     http::StatusCode
/// };
/// use axum_extra::routing::RouterExt;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home;
///
/// // Define routes and associate them with handlers
/// routes! {
///     name => pub AllRoutes, // This makes `AllRoutes` public
///     "/" => Home,
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
///     // Create a Router instance using the `routes()`
///     // method provided by the `AllRoutes` struct
///     let r: Router = AllRoutes::routes();
/// }
/// ```
///
/// # With State
/// ```rust
/// use better_routes::{routes, method_helper};
/// use axum::{
///     Router
/// };
/// use axum_extra::routing::RouterExt;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home;
///
/// #[derive(Clone)]
/// struct AppState;
///
/// // Define routes and specify the application state
/// routes! {
///     name => AllRoutes,
///     state => AppState,
///     "/" => Home
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
///     // Create a Router instance with state using the `routes()`
///     // method provided by the `AllRoutes` struct
///     let r: Router<AppState> = AllRoutes::routes();
/// }
/// ```
///
/// # With Global Rejection
/// ```rust
/// use better_routes::{routes, method_helper};
/// use axum::{
///     Router,
///     http::StatusCode,
///     response::{IntoResponse, Response},
///     extract::rejection::PathRejection
/// };
/// use axum_extra::routing::RouterExt;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home {
///     id: usize
/// }
///
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
/// // Define routes and specify global rejection handling
/// routes! {
///     name => AllRoutes,
///     rejection => GlobalRejection,
///     "/:id" => Home
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
///     // Create a Router instance using the `routes()`
///     // method provided by the `AllRoutes` struct
///     let r: Router = AllRoutes::routes();
/// }
/// ```
///
/// # With Route-Specific Rejection
/// ```rust
/// use better_routes::{routes, method_helper};
/// use axum::{
///     Router,
///     http::StatusCode,
///     response::{IntoResponse, Response},
///     extract::rejection::PathRejection
/// };
/// use axum_extra::routing::RouterExt;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home {
///     id: usize
/// }
///
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
/// // Define routes and specify route-specific rejection handling
/// routes! {
///     name => AllRoutes,
///     "/:id" => Home => HomeRejection
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
///     // Create a Router instance using the `routes()`
///     // method provided by the `AllRoutes` struct
///     let r: Router = AllRoutes::routes();
/// }
/// ```
pub use better_routes_macros::routes;

/// The [`MethodHandlers`] trait defines a list of HTTP methods that a route can handle.
///
/// Note: Instead of implementing it directly, you should use the [`method_helper`]
/// macro to generate the necessary implementations.
pub trait MethodHandlers {
    const METHODS: &'static [axum::http::Method];
}
