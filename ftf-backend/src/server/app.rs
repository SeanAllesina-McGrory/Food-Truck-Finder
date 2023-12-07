use crate::server::handlers;
use crate::server::state::AppState;
use anyhow::{anyhow, Result};
use axum::{routing::get, Router};
use std::env;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tower_http::cors::{Any, CorsLayer};

pub async fn make_app() -> Result<Router> {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .layer(cors)
        .route("/vendor/get", get(handlers::vendor_get))
        .route("/vendor/add", get(handlers::vendor_add))
        .route("/vendor/remove", get(handlers::vendor_remove))
        .route("/event/get", get(handlers::event_get))
        .route("/event/add", get(handlers::event_add))
        .route("/event/remove", get(handlers::event_remove))
        .route("/menu/get", get(handlers::menu_get))
        .route("/menu/add", get(handlers::menu_add))
        .route("/menu/remove", get(handlers::menu_remove))
        .route("/item/get", get(handlers::item_get))
        .route("/item/add", get(handlers::item_add))
        .route("/item/remove", get(handlers::item_remove))
        .with_state(AppState {
            db: match db_connect().await {
                Ok(db) => db,
                Err(err) => return Err(anyhow!(err)), // FIX : The database could not be created, if this happens a
                                                      // panic is undesirable but likely, add correcting code
                                                      // later
            },
        });
    Ok(app)
}

pub async fn db_connect() -> Result<Surreal<Client>> {
    let db;

    match get_db_creds() {
        Ok(response) => {
            db = Surreal::new::<Ws>("localhost:8000").await?;
            db.signin(Root {
                username: &response[0],
                password: &response[1],
            })
            .await?;
        }
        Err(_) => return Err(anyhow!("Failed to connect to database")),
    }

    // Sets up the database info, will just use defaults if the env variables aren't set
    // Including this in the source shouldn't be a security risk since its just database names
    db.use_ns(match env::var("DBNS") {
        Ok(namespace) => namespace.to_owned(),
        Err(_) => "food_truck_finder".to_string(),
    })
    .use_db(match env::var("DBNM") {
        Ok(database_name) => database_name.to_owned(),
        Err(_) => "ftf_db".to_string(),
    })
    .await?;

    Ok(db)
}

fn get_db_creds() -> Result<Vec<String>> {
    let username = match env::var("DBUN") {
        Ok(uname) => uname,
        Err(_) => {
            return Err(anyhow!(
                "Database username environment variable could not be resolved"
            ))
        }
    };
    let password = match env::var("DBPW") {
        Ok(pword) => pword,
        Err(_) => {
            return Err(anyhow!(
                "Database password environment variable could not be resolved"
            ))
        }
    };

    Ok(vec![username, password])
}
