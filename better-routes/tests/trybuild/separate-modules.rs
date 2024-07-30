mod rejection {
    use axum::response::{IntoResponse, Response};

    #[derive(Default)]
    pub struct GlobalRejection;

    impl IntoResponse for GlobalRejection {
        fn into_response(self) -> Response {
            todo!()
        }
    }

    #[derive(Default)]
    pub struct FooRejection;

    impl IntoResponse for FooRejection {
        fn into_response(self) -> Response {
            todo!()
        }
    }
}

mod state {
    #[derive(Clone)]
    pub struct State {}
}

mod route {
    use better_routes::routes;

    routes! {
        State => super::state::State,
        Rejection => super::rejection::GlobalRejection,
        "/foo" => pub struct Foo; => super::rejection::FooRejection,
        "/bar" => pub struct Bar;,
    }
}

mod baz {
    use axum_extra::routing::RouterExt;
    use better_routes::method_helper;

    #[method_helper(super::state::State)]
    impl super::route::Foo {
        #[get]
        async fn foo(self) {}
    }

    #[method_helper(super::state::State)]
    impl super::route::Bar {
        #[get]
        async fn bar(self) {}
    }
}

fn main() {}
