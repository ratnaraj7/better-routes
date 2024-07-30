use axum::body::Body;
use axum::extract::rejection::PathRejection;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_extra::routing::RouterExt;
use better_routes::{method_helper, routes};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[test]
fn test_all() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/should-pass.rs");
    t.pass("tests/trybuild/empty-macro.rs");
    t.pass("tests/trybuild/separate-modules.rs");

    t.compile_fail("tests/trybuild/double-state.rs");
    t.compile_fail("tests/trybuild/double-rejection.rs");
    t.compile_fail("tests/trybuild/non-async-handler.rs");
    t.compile_fail("tests/trybuild/no-handler.rs");
    t.compile_fail("tests/trybuild/no-arg.rs");
    t.compile_fail("tests/trybuild/non-self-arg.rs");
    t.compile_fail("tests/trybuild/without-routes.rs");
}

fn make_request(path: &'static str, method: &'static str) -> Request<Body> {
    Request::builder()
        .uri(path)
        .method(method)
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn test_routes() {
    routes! {
        "/" => struct Home;
    }

    #[method_helper]
    impl Home {
        #[get]
        async fn home(self) {}
    }

    let router = router();

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
async fn test_routes_with_global_rejection() {
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

    routes! {
        Rejection => GlobalRejection,
        "/:id" => struct Home {id : usize}
    }

    #[method_helper]
    impl Home {
        #[post]
        async fn home(self) {}
    }

    let router = router();

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
async fn test_routes_with_global_rejection_and_path_rejection() {
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

    routes! {
        Rejection => GlobalRejection,
        "/:id" => struct Home {id : usize},
        "/user/:id" => struct User {id : usize} => UserRejection
    }

    #[method_helper]
    impl Home {
        #[post]
        async fn home(self) {}
    }

    #[method_helper]
    impl User {
        #[post]
        async fn user(self) {}
    }

    let router = router();

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
