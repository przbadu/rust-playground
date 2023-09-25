#![allow(unused)]

use std::{fmt::format, net::SocketAddr};

use axum::{
    extract::Path,
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().merge(routes_hello());

    // region:     -- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server listening on http://{addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion: -- Start Server
}

// region:    -- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));
}
// endregion: -- Routes Hello

// region:    --- Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g: `/hello?name=John`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"));
}

// e.g., `/hello/john`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"));
}
// endregion: --- Handler Hello
