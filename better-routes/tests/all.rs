use axum::body::Body;
use axum::extract::rejection::PathRejection;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::routing::RouterExt;
use better_routes::routes;
use http_body_util::BodyExt;
use serde::Deserialize;
use tower::ServiceExt;

#[test]
fn test_all() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/pass/*.rs");
    t.compile_fail("tests/trybuild/fail/*.rs");
}

fn make_request(path: &'static str, method: &'static str) -> Request<Body> {
    Request::builder()
        .uri(path)
        .method(method)
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn should_only_allow_get_request() {
    #[derive(Deserialize)]
    struct HomePath;
    async fn home(_: HomePath) {}
    routes! {
        name => AllRoutes,
        "/" => HomePath {
            get => home
        },
    }
    let router = AllRoutes::routes();
    let get_req = make_request("/", "GET");
    let post_req = make_request("/", "POST");
    let put_req = make_request("/", "PUT");
    let delete_req = make_request("/", "DELETE");
    let patch_req = make_request("/", "PATCH");
    let get_res = router.clone().oneshot(get_req).await.unwrap();
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let put_res = router.clone().oneshot(put_req).await.unwrap();
    let delete_res = router.clone().oneshot(delete_req).await.unwrap();
    let patch_res = router.clone().oneshot(patch_req).await.unwrap();
    assert_eq!(get_res.status(), StatusCode::OK);
    assert_eq!(post_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(put_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(delete_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(patch_res.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn should_only_allow_post_request() {
    #[derive(Deserialize)]
    struct HomePath;
    async fn home(_: HomePath) {}
    routes! {
        name => AllRoutes,
        "/" => HomePath {
            post => home
        },
    }
    let router = AllRoutes::routes();
    let get_req = make_request("/", "GET");
    let post_req = make_request("/", "POST");
    let put_req = make_request("/", "PUT");
    let delete_req = make_request("/", "DELETE");
    let patch_req = make_request("/", "PATCH");
    let get_res = router.clone().oneshot(get_req).await.unwrap();
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let put_res = router.clone().oneshot(put_req).await.unwrap();
    let delete_res = router.clone().oneshot(delete_req).await.unwrap();
    let patch_res = router.clone().oneshot(patch_req).await.unwrap();
    assert_eq!(get_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(post_res.status(), StatusCode::OK);
    assert_eq!(put_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(delete_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(patch_res.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn should_only_allow_put_request() {
    #[derive(Deserialize)]
    struct HomePath;
    async fn home(_: HomePath) {}
    routes! {
        name => AllRoutes,
        "/" => HomePath {
            put => home
        },
    }
    let router = AllRoutes::routes();
    let get_req = make_request("/", "GET");
    let post_req = make_request("/", "POST");
    let put_req = make_request("/", "PUT");
    let delete_req = make_request("/", "DELETE");
    let patch_req = make_request("/", "PATCH");
    let get_res = router.clone().oneshot(get_req).await.unwrap();
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let put_res = router.clone().oneshot(put_req).await.unwrap();
    let delete_res = router.clone().oneshot(delete_req).await.unwrap();
    let patch_res = router.clone().oneshot(patch_req).await.unwrap();
    assert_eq!(get_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(post_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(put_res.status(), StatusCode::OK);
    assert_eq!(delete_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(patch_res.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn should_only_allow_del_request() {
    #[derive(Deserialize)]
    struct HomePath;
    async fn home(_: HomePath) {}
    routes! {
        name => AllRoutes,
        "/" => HomePath{
            delete => home
        },
    }
    let router = AllRoutes::routes();
    let get_req = make_request("/", "GET");
    let post_req = make_request("/", "POST");
    let put_req = make_request("/", "PUT");
    let delete_req = make_request("/", "DELETE");
    let patch_req = make_request("/", "PATCH");
    let get_res = router.clone().oneshot(get_req).await.unwrap();
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let put_res = router.clone().oneshot(put_req).await.unwrap();
    let delete_res = router.clone().oneshot(delete_req).await.unwrap();
    let patch_res = router.clone().oneshot(patch_req).await.unwrap();
    assert_eq!(get_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(post_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(put_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(delete_res.status(), StatusCode::OK);
    assert_eq!(patch_res.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn should_only_allow_patch_request() {
    #[derive(Deserialize)]
    struct HomePath;
    async fn home(_: HomePath) {}
    routes! {
        name => AllRoutes,
        "/" => HomePath{
            patch => home
        },
    }
    let router = AllRoutes::routes();
    let get_req = make_request("/", "GET");
    let post_req = make_request("/", "POST");
    let put_req = make_request("/", "PUT");
    let delete_req = make_request("/", "DELETE");
    let patch_req = make_request("/", "PATCH");
    let get_res = router.clone().oneshot(get_req).await.unwrap();
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let put_res = router.clone().oneshot(put_req).await.unwrap();
    let delete_res = router.clone().oneshot(delete_req).await.unwrap();
    let patch_res = router.clone().oneshot(patch_req).await.unwrap();
    assert_eq!(get_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(post_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(put_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(delete_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(patch_res.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_reject_with_global_rejection() {
    #[derive(Default)]
    struct GlobalRejection;
    impl From<PathRejection> for GlobalRejection {
        fn from(_: PathRejection) -> Self {
            GlobalRejection
        }
    }
    impl IntoResponse for GlobalRejection {
        fn into_response(self) -> Response {
            StatusCode::NOT_FOUND.into_response()
        }
    }
    #[derive(Deserialize)]
    struct HomePath {
        id: usize,
    }
    async fn home(_: HomePath) {}
    routes! {
        name => AllRoutes,
        rejection => GlobalRejection,
        "/:id" => HomePath{
            post => home
        },
    }
    let router = AllRoutes::routes();
    let get_req = make_request("/123", "GET");
    let post_req = make_request("/123", "POST");
    let invalid_post_req = make_request("/invalid-id", "POST");
    let put_req = make_request("/123", "PUT");
    let delete_req = make_request("/123", "DELETE");
    let patch_req = make_request("/123", "PATCH");
    let get_res = router.clone().oneshot(get_req).await.unwrap();
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let invalid_post_res = router.clone().oneshot(invalid_post_req).await.unwrap();
    let put_res = router.clone().oneshot(put_req).await.unwrap();
    let delete_res = router.clone().oneshot(delete_req).await.unwrap();
    let patch_res = router.clone().oneshot(patch_req).await.unwrap();
    assert_eq!(get_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(post_res.status(), StatusCode::OK);
    assert_eq!(invalid_post_res.status(), StatusCode::NOT_FOUND);
    assert_eq!(put_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(delete_res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(patch_res.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn should_reject_with_route_specific_rejection() {
    #[derive(Default)]
    struct GlobalRejection;
    impl From<PathRejection> for GlobalRejection {
        fn from(_: PathRejection) -> Self {
            GlobalRejection
        }
    }
    impl IntoResponse for GlobalRejection {
        fn into_response(self) -> Response {
            "global".into_response()
        }
    }
    #[derive(Default)]
    struct UserRejection;
    impl From<PathRejection> for UserRejection {
        fn from(_: PathRejection) -> Self {
            UserRejection
        }
    }
    impl IntoResponse for UserRejection {
        fn into_response(self) -> Response {
            "user".into_response()
        }
    }
    #[derive(Deserialize)]
    struct HomePath {
        id: usize,
    }
    async fn home(_: HomePath) {}
    #[derive(Deserialize)]
    struct UserPath {
        id: usize,
    }
    async fn user(_: UserPath) {}
    routes! {
        name => AllRoutes,
        rejection => GlobalRejection,
        "/:id" => HomePath{
            post => home
        },
        "/user/:id" => rejection UserRejection => UserPath{
            post => user
        }
    }
    let router = AllRoutes::routes();
    let post_req = make_request("/123", "POST");
    let invalid_post_req = make_request("/invalid-id", "POST");
    let user_post_req = make_request("/user/123", "POST");
    let invalid_user_post_req = make_request("/user/invalid-id", "POST");
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let invalid_post_res = router.clone().oneshot(invalid_post_req).await.unwrap();
    let user_post_res = router.clone().oneshot(user_post_req).await.unwrap();
    let invalid_user_post_res = router.clone().oneshot(invalid_user_post_req).await.unwrap();
    assert_eq!(post_res.status(), StatusCode::OK);
    let body = invalid_post_res
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    assert_eq!(&body[..], b"global");
    assert_eq!(user_post_res.status(), StatusCode::OK);
    let body = invalid_user_post_res
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    assert_eq!(&body[..], b"user");
}

#[tokio::test]
async fn should_capture_wildcard() {
    #[derive(Deserialize)]
    struct HomePath {
        other: String,
    }
    async fn home(home_path: HomePath) -> String {
        home_path.other
    }
    routes! {
        name => AllRoutes,
        "/*other" => HomePath{
            post => home
        }
    }
    let router = AllRoutes::routes();
    let post_req = make_request("/hello/world", "POST");
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let body = post_res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"hello/world");
}

#[tokio::test]
async fn all() {
    #[derive(Deserialize)]
    struct Home {
        id: usize,
        rest: String,
    }

    async fn get_some(_: About) {}
    async fn get_som(_: Home) {}

    #[derive(Deserialize)]
    struct About;

    async fn get_about(_: About) {}

    routes! {
        name => AllRoutes,
        "/:id/*rest" => Home {
            get => get_som,
            post => get_som
        },
        "/about" => About {
            get => get_about
        },
    }

    let _r = AllRoutes::routes();
}
