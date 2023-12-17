use crate::server::{handlers, state::AppState};
use anyhow::{anyhow, Result};
use axum::body::Body;
#[allow(unused_imports)]
use axum::{
    http::{Request, Response, StatusCode},
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
#[warn(unused_imports)]
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tower_http::cors::{Any, CorsLayer};

pub async fn make_app() -> Result<Router> {
    let cors = CorsLayer::new().allow_origin(Any);

    // The valid endpoints for the API
    // Endpoints which can be accessed without authorization marked with *
    // All others require preauthorization by means of a header with an auth token
    //  this auth token allows accessing and modification of data owned by the
    //  authorized party
    //  Even when authorization is given, all requests are checked to verify they
    //      only operate on owned or free data.
    let endpoints = Router::new()
        // Routes dealing with vendor resources
        // Get -> All vendors*
        // Post -> Creates a new vendor
        //          This is a special route which is governed by a seperate authorization agent
        //          Since a new vendor cannot verify themselves, and a vendor shouldn't be able to
        //              create a new vendor
        //          This route takes an Facebook Oauth string and can be freely called
        //          WARNING: This route needs to do verification to ensure it is not used by
        //          malicious individuals
        //
        // Else -> 404
        .route(
            "/vendors",
            get(handlers::get_vendors).post(handlers::post_vendor),
        )
        // Routes dealing with general event resources
        // Get -> All events*
        // Else -> 404
        .route("/events", get(handlers::get_events))
        // Routes dealing with specific vendor resources
        // Get -> Specific vendor*
        // Delete -> Specific vendor
        //            WARNING: This route should not be accessable, it is here for parity
        //            Admin authentication is necessary to call this route
        //            Vendors won't be allowed to delete their own accounts
        //
        // Patch -> Specific vendor
        // Else -> 404
        .route(
            "/vendors/:vendor_id",
            get(handlers::get_vendors)
                .delete(handlers::delete_vendor)
                .patch(handlers::patch_vendor),
        )
        // Routes dealing with specific events
        // Get -> Specific event*
        // Delete -> Specific event belonging to authorized vendor
        // Patch -> Specific event belong to authorized vendor
        // Else -> 404
        .route(
            "/events/:event_id",
            get(handlers::get_events)
                .delete(handlers::delete_event)
                .patch(handlers::patch_event),
        )
        // Routes dealing with specific menus
        // Get -> Specific menu
        // Delete -> Specific menu belonging to authorized vendor
        // Patch -> Specific menu belonging to authorized vendor
        // Else -> 404
        .route(
            "/menus/:menu_id",
            get(handlers::get_menus)
                .delete(handlers::delete_menu)
                .patch(handlers::patch_menu),
        )
        // Routes dealing with specific items
        // Get -> Specific item
        // Delete -> Specific item belonging to authorized vendor
        // Patch -> Specific item belonging to authorized vendor
        // Else -> 404
        .route(
            "/items/:item_id",
            get(handlers::get_items)
                .delete(handlers::delete_item)
                .patch(handlers::patch_item),
        )
        // Routes dealing with general groups belonging to a specific vendor
        // Get -> All events belonging to specific vendor
        // Post -> Creates new event for authorized vendor
        // Else -> 404
        .route(
            "/vendors/:vendor_id/events",
            get(handlers::get_events).post(handlers::post_event),
        )
        // Get -> All menus belonging to specific vendor
        // Post -> Creates a new menu for authorized vendor
        // Else -> 404
        .route(
            "/vendors/:vendor_id/menus",
            get(handlers::get_menus).post(handlers::post_menu),
        )
        // Get -> All items belonging to specific vendor
        // Post -> Creates a new item for authorized vendor
        // Else -> 404
        .route(
            "/vendors/:vendor_id/items",
            get(handlers::get_items).post(handlers::post_item),
        );
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
        })
        .layer(middleware::from_fn(authorize));
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

async fn authorize(
    request: Request<Body>,
    next: middleware::Next,
) -> Result<Response<Body>, StatusCode> {
    if let Some(auth_token) = request.headers().get("AUTH_TOKEN") {
        if auth_token.to_owned() == "1234" {
            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
