mod database;
mod server;
mod utils;

use crate::server::app;

#[tokio::main]
async fn main() {
    let app = match app::make_app().await {
        Ok(app) => app,
        // Failing here causes a panic since if the api cant talk to the server its pointless for
        // the program to continue executing
        // However this is not desirable functionality
        // While a panic will only occur at launch it may be an issue, especially after reboots if
        // the SurrealDB server takes longer to launch than the API
        // TODO: fix later
        Err(err) => panic!(
            "The server could not be created due to the following err\n{}",
            err
        ),
    };
    // region:      -- Start Server

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("-> Listening on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    // endregion:   -- Start Server
}
