use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/hello", get(handler_hello))
        .layer(cors);

    // region:      -- Start Server

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("-> Listening on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    // endregion:   -- Start Server
}

// region:      -- Handlers

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("Cruel World!");

    Html(format!("Hello, <strong>{name}</strong>"))
}
// endregion:   -- Handlers
