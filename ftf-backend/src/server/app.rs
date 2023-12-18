use crate::server::{handlers, state::AppState};
use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::Redirect,
};
#[allow(unused_imports)]
use axum::{
    http::{Method, Request, Response, StatusCode},
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{self, HashMap},
};
#[warn(unused_imports)]
use std::{env, iter::Map};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tower_http::cors::{Any, CorsLayer};

use super::{handlers::Record, state};

pub async fn make_app() -> Result<Router> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_origin(Any);

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
        .route("/auth", get(authorize))
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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TokenStore {
    token_hash: Cow<'static, str>,
    expires: Cow<'static, i64>,
    venodr_id: Cow<'static, str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

#[derive(Debug, Deserialize)]
struct AuthorizeParams {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct Email {
    email: String,
    id: String,
}

// This is where the code which authorizes a user will go
// For now it will take the header: "Authorization" and store it in the database
// When a user sends a request, their provided Authorization header will be checked
//      against the database and if the provided token exists they will be considered authenticated
async fn authorize(
    Query(params): Query<AuthorizeParams>,
    State(state): State<state::AppState>,
) -> Redirect {
    dbg!(&params);
    let code = params.code;

    if let Ok(verify_code_url) = env::var("VERIFY_CODE_URL_TEST") {
        dbg!(&code);
        let url = format!("{verify_code_url}{code}");
        println!("{}", &url);
        let response_result = reqwest::get(url).await;
        let response = match response_result {
            Ok(response) => response,
            Err(err) => return Redirect::to("http://localhost:8081"),
        };
        let token_result = response.json::<Token>().await;
        let mut token = match token_result {
            Ok(token) => token,
            Err(err) => return Redirect::to("http://localhost:8081"),
        };

        let response_result = reqwest::get(format!(
            "https://graph.facebook.com/v18.0/me?fields=email&access_token={}",
            token.access_token
        ))
        .await;
        let response = match response_result {
            Ok(response) => response,
            Err(err) => return Redirect::to("http://localhost:8081"),
        };

        let email_result = response.json::<Email>().await;
        let email = match email_result {
            Ok(token) => token,
            Err(err) => return Redirect::to("http://localhost:8081"),
        };

        println!("{}", email.email);

        let params: argon2::Params = argon2::Params::new(512, 32, 16, 32.into()).unwrap();
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2i,
            argon2::Version::V0x13,
            params.clone(),
        );

        let token_hash = argon2
            .hash_password(token.access_token.as_bytes(), &salt)
            .unwrap()
            .to_string();

        token.access_token = token_hash;
        let records_result: Result<Vec<Token>, surrealdb::Error> =
            state.db.create("tokens").content(TokenStore{
                token_hash: token_hash.into(),
                expires: chrono::Utc::now().timestamp() + token.expires_in,
                vendor_id:

            }).await;
        let records = match records_result {
            Ok(_) => {
                println!("User was authenticated");
                return Redirect::to("http://localhost:8081/auth");
            }
            Err(err) => return Redirect::to("http://localhost:8081"),
        };
    }

    Redirect::to("http://localhost:8081")
}
