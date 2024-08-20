use axum::body::Body;
use axum::extract::rejection::PathRejection;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_extra::routing::RouterExt;
use better_routes::{method_helper, routes};
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
    routes! {
        name => AllRoutes,
        "/" => HomePath,
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[get]
        async fn home(self) {}
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
    routes! {
        name => AllRoutes,
        "/" => HomePath,
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[post]
        async fn home(self) {}
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
    routes! {
        name => AllRoutes,
        "/" => HomePath,
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[put]
        async fn home(self) {}
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
    routes! {
        name => AllRoutes,
        "/" => HomePath,
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[delete]
        async fn home(self) {}
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
    routes! {
        name => AllRoutes,
        "/" => HomePath,
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[patch]
        async fn home(self) {}
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
    routes! {
        name => AllRoutes,
        rejection => GlobalRejection,
        "/:id" => HomePath,
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[post]
        async fn home(self) {}
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
    #[derive(Deserialize)]
    struct UserPath {
        id: usize,
    }
    routes! {
        name => AllRoutes,
        rejection => GlobalRejection,
        "/:id" => HomePath,
        "/user/:id" => UserPath => UserRejection
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[post]
        async fn home(self) {}
    }
    #[method_helper]
    #[allow(dead_code)]
    impl UserPath {
        #[post]
        async fn user(self) {}
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
    routes! {
        name => AllRoutes,
        "/*other" => HomePath
    }
    #[method_helper]
    #[allow(dead_code)]
    impl HomePath {
        #[post]
        async fn home(self) -> String {
            self.other
        }
    }
    let router = AllRoutes::routes();
    let post_req = make_request("/hello/world", "POST");
    let post_res = router.clone().oneshot(post_req).await.unwrap();
    let body = post_res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"hello/world");
}
