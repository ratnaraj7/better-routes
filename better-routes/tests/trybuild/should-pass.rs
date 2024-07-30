use axum::response::{IntoResponse, Response};

#[derive(Default)]
struct GlobalRejection;

impl IntoResponse for GlobalRejection {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Default)]
struct FooRejection;

impl IntoResponse for FooRejection {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Clone)]
struct State {}

mod pass1 {
    use axum_extra::routing::RouterExt;
    use better_routes::method_helper;
    use better_routes::routes;

    #[method_helper(super::State)]
    impl Foo {
        #[get]
        async fn foo(self) {}
    }

    #[method_helper(super::State)]
    impl Bar {
        #[get]
        async fn bar(self) {}
    }

    routes! {
        State => super::State,
        Rejection => super::GlobalRejection,
        "/foo" => struct Foo; => super::FooRejection,
        "/bar" => struct Bar;,
    }
}

mod pass2 {
    use axum_extra::routing::RouterExt;
    use better_routes::method_helper;
    use better_routes::routes;

    #[method_helper(super::State)]
    impl Foo {
        #[get]
        async fn foo(self) {}
    }

    #[method_helper(super::State)]
    impl Bar {
        #[get]
        async fn bar(self) {}
    }

    routes! {
        State => super::State,
        Rejection => super::GlobalRejection,
        "/foo" => struct Foo;,
        "/bar" => struct Bar;,
    }
}

mod pass3 {
    use axum_extra::routing::RouterExt;
    use better_routes::method_helper;
    use better_routes::routes;

    #[method_helper]
    impl Foo {
        #[get]
        async fn foo(self) {}
    }

    #[method_helper]
    impl Bar {
        #[get]
        async fn bar(self) {}
    }

    routes! {
        "/foo" => struct Foo;,
        "/bar" => struct Bar;,
    }
}

mod pass4 {
    use axum_extra::routing::RouterExt;
    use better_routes::method_helper;
    use better_routes::routes;

    #[method_helper(super::State)]
    impl Foo {
        #[get]
        async fn foo(self) {}
    }

    #[method_helper(super::State)]
    impl Bar {
        #[get]
        async fn bar(self) {}
    }

    routes! {
        State => super::State,
        "/foo" => struct Foo;,
        "/bar" => struct Bar;,
    }
}

mod pass5 {
    use axum_extra::routing::RouterExt;
    use better_routes::method_helper;
    use better_routes::routes;

    #[method_helper(super::State)]
    impl Foo {
        #[get]
        async fn foo(self) {}
    }

    #[method_helper(super::State)]
    impl Bar {
        #[get]
        async fn bar(self) {}
    }

    routes! {
        State => super::State,
        "/foo/:id" => struct Foo { id: String },
        "/bar/:id" => struct Bar { id: String },
    }
}

fn main() {}
