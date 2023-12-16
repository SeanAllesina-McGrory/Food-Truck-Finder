use crate::server::{handlers, state::AppState};
use anyhow::{anyhow, Result};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tower_http::cors::{Any, CorsLayer};

pub async fn make_app() -> Result<Router> {
    let cors = CorsLayer::new().allow_origin(Any);

    let endpoints = Router::new()
        // Post Routes
        .route(
            "/vendors",
            get(handlers::get_vendors).post(handlers::post_vendor),
        )
        .route("/events", get(handlers::get_events))
        .route(
            "/vendors/:vendor_id",
            get(handlers::get_vendors)
                .delete(handlers::delete_vendor)
                .patch(handlers::patch_vendor),
        )
        .route(
            "/events/:event_id",
            get(handlers::get_events)
                .post(handlers::post_event)
                .delete(handlers::delete_event)
                .patch(handlers::patch_event),
        )
        .route(
            "/menus/:menu_id",
            get(handlers::get_menus)
                .post(handlers::post_menu)
                .delete(handlers::delete_menu)
                .patch(handlers::patch_menu),
        )
        .route(
            "/items/:item_id",
            get(handlers::get_items)
                .post(handlers::post_item)
                .delete(handlers::delete_item)
                .patch(handlers::patch_item),
        )
        .route("/vendors/:vendor_id/events", get(handlers::get_events))
        .route("/vendors/:vendor_id/menus", get(handlers::get_menus))
        .route("/vendors/:vendor_id/items", get(handlers::get_items));
    let app = Router::new()
        .layer(cors)
        .nest("/v1", endpoints)
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
