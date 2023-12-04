mod database;
pub mod server;

#[tokio::main]
async fn main() {
    let app = server::make_app();
    // region:      -- Start Server

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("-> Listening on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    // endregion:   -- Start Server
}
