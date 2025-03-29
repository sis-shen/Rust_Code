// use axum::{
//     routing::get,
//     Router,
// };

// #[tokio::main]
// async fn main() {
//     // build our application with a single route
//     let app = Router::new().route("/", get(|| async { "Hello, World!" }));

//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

use axum::{Router, routing::get};

// our router


#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// which calls one of these handlers
async fn root() -> &'static str{"hello world"}
async fn get_foo()-> &'static str {"get_foo"}
async fn post_foo()-> &'static str {"post_foo"}
async fn foo_bar()->&'static str {"foo_bar"}