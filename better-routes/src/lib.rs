//! Better Routes
//!
//! `better_routes` is a macro for centralizing all routes in an Axum application,
//! making routing type-safe, maintainable, and less error-prone. It allows you to define
//! routes, their handlers, and rejections in a single place, simplifying your application's
//! routing logic.
//!
//! # Examples
//!
//! For various usage examples, check out the [examples directory](https://github.com/ratnaraj7/better-routes/tree/main/examples) in the repository.
//!
//! # Example
//!
//! ```rust,no_run
//! use axum::{
//!     extract::rejection::PathRejection, http::StatusCode, response::IntoResponse, Json, Router,
//! };
//! use axum_extra::routing::RouterExt;
//! use better_routes::routes;
//! use serde::Deserialize;
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
//! pub struct AppState;
//!
//! // Define a struct for products
//! #[derive(Deserialize)]
//! struct Product {
//!     id: usize,
//! }
//!
//! async fn get_product(product: Product) -> Json<String> {
//!     println!("Product ID: {}", product.id);
//!     Json(format!("Product ID: {}", product.id))
//! }
//!
//! async fn create_product(product: Product) -> Json<String> {
//!     println!("Creating product with ID: {}", product.id);
//!     Json(format!("Created product with ID: {}", product.id))
//! }
//!
//! async fn update_product(product: Product) -> Json<String> {
//!     println!("Updating product with ID: {}", product.id);
//!     Json(format!("Updated product with ID: {}", product.id))
//! }
//!
//! async fn delete_product(product: Product) -> Json<String> {
//!     println!("Deleting product with ID: {}", product.id);
//!     Json(format!("Deleted product with ID: {}", product.id))
//! }
//!
//! async fn patch_product(product: Product) -> Json<String> {
//!     println!("Patching product with ID: {}", product.id);
//!     Json(format!("Patched product with ID: {}", product.id))
//! }
//!
//! // Define a struct for users
//! #[derive(Deserialize)]
//! struct User {
//!     id: usize,
//! }
//!
//! async fn get_user(user: User) -> Json<String> {
//!     println!("User ID: {}", user.id);
//!     Json(format!("User ID: {}", user.id))
//! }
//!
//! async fn create_user(user: User) -> Json<String> {
//!     println!("Creating user with ID: {}", user.id);
//!     Json(format!("Created user with ID: {}", user.id))
//! }
//!
//! async fn update_user(user: User) -> Json<String> {
//!     println!("Updating user with ID: {}", user.id);
//!     Json(format!("Updated user with ID: {}", user.id))
//! }
//!
//! async fn delete_user(user: User) -> Json<String> {
//!     println!("Deleting user with ID: {}", user.id);
//!     Json(format!("Deleted user with ID: {}", user.id))
//! }
//!
//! async fn patch_user(user: User) -> Json<String> {
//!     println!("Patching user with ID: {}", user.id);
//!     Json(format!("Patched user with ID: {}", user.id))
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
//!     "/product/:id" => Product{
//!         get => get_product, // Define route for getting product
//!         post => create_product, // Define route for creating product
//!         put => update_product, // Define route for updating product
//!         patch => patch_product, // Define route for patching product
//!         delete => delete_product, // Define route for deleting product
//!     },
//!     "/user/:id" => rejection UserRejection => User{
//!         get => get_user, // Define route for getting user
//!         post => create_user, // Define route for creating user
//!         put => update_user, // Define route for updating user
//!         patch => patch_user, // Define route for patching user
//!         delete => delete_user, // Define route for deleting user
//!     },
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

/// Define routes using the [`routes!`] macro.
///
/// The [`routes!`] macro generates a struct, using the name provided, such as `AllRoutes` below,
/// that implements the `routes()` method. This method creates a
/// `Router` instance configured with the routes defined in the macro.
/// You can control the visibility of the generated struct and methods using
/// visibility modifiers.
///
/// # Example
/// ```rust
/// use axum::Router;
/// use axum_extra::routing::RouterExt;
/// use better_routes::routes;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home;
///
/// async fn index(_: Home) {}
///
/// // Define routes and associate them with handlers
/// routes! {
///     name => pub AllRoutes, // This makes `AllRoutes` public
///     "/" => Home{
///         get => index
///     },
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
/// use axum::Router;
/// use axum_extra::routing::RouterExt;
/// use better_routes::routes;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home;
///
/// async fn index(_: Home) {}
///
/// #[derive(Clone)]
/// struct AppState;
///
/// // Define routes and specify the application state
/// routes! {
///     name => AllRoutes,
///     state => AppState,
///     "/" => Home {
///         get => index
///     }
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
/// use axum::{
///     extract::rejection::PathRejection,
///     http::StatusCode,
///     response::{IntoResponse, Response},
///     Router,
/// };
/// use axum_extra::routing::RouterExt;
/// use better_routes::routes;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home {
///     id: usize
/// }
///
/// async fn index(_: Home) {}
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
///     "/:id" => Home {
///         get => index
///     }
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
/// use axum::{
///     extract::rejection::PathRejection,
///     http::StatusCode,
///     response::{IntoResponse, Response},
///     Router,
/// };
/// use axum_extra::routing::RouterExt;
/// use better_routes::routes;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Home {
///     id: usize
/// }
///
/// async fn index(_: Home) {}
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
///     "/:id" => rejection HomeRejection => Home {
///         get => index
///     }
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
