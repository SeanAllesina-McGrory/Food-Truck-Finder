use anyhow::{anyhow, Result};
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;
use std::fmt;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tower_http::cors::{Any, CorsLayer};

pub fn make_app() -> Router {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .layer(cors)
        .route("/vendor/add", get(vendor_add))
        .route("/vendor/remove", get(vendor_remove))
        .route("/event/add", get(event_add))
        .route("/event/remove", get(event_remove))
        .route("/menu/add", get(menu_add))
        .route("/menu/remove", get(menu_remove))
        .route("/item/add", get(item_add))
        .route("/item/remove", get(item_remove));
    app
}

async fn db_connect() -> Result<Surreal<Client>> {
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

// region:      -- Handlers

async fn vendor_add(Query(params): Query<VendorAddParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_add - {params:?}", "HANDLER");

    let db = match db_connect().await {
        Ok(db) => db,
        Err(err) => return Html(format!("{}", err)),
    };

    let vendor: Vec<Vendor> = match db
        .create("vendor")
        .content(Vendor {
            name: params.name.into(),
            auth_token: params.auth_token.into(),
            description: params.description.clone().unwrap_or("".to_string()).into(),
            vendor_type: params.vendor_type.clone().unwrap_or("".to_string()).into(),
            email: params.email.clone().unwrap_or("".to_string()).into(),
            phone_number: params.phone_number.clone().unwrap_or("".to_string()).into(),
            website: params.website.clone().unwrap_or("".to_string()).into(),
            events: vec![],
            menu: vec![],
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Html(format!("{:?}", err)),
    };

    Html(format!("{:?}", vendor))
}

async fn vendor_remove(Query(params): Query<VendorRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_remove - {params:?}", "HANDLER");

    let vendor_id = params.vendor_id;

    Html(format!("vendor_id: {vendor_id}"))
}

async fn event_add(Query(params): Query<EventAddParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler event_add - {params:?}", "HANDLER");

    let datetime = params.datetime;
    let location = params.location;
    let repetition = params.repetition;
    let vendor_id = params.vendor_id;

    let event = Event {
        name: Cow::from(""),
        datetime: Cow::from(datetime.clone()),
        location: Cow::from(location.clone()),
        repeat_schedule: Cow::from(repetition.clone()),
        repeat_end: Cow::from(datetime.clone()),
        menu: None,
        vendor: None,
    };

    Html(format!(
        "datetime: {0}\nlocation: {1}\nrepetition: {2}\nvendor_id: {3:?}",
        event.datetime, event.location, event.repeat_schedule, event.vendor
    ))
}

async fn event_remove(Query(params): Query<EventRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler event_remove - {params:?}", "HANDLER");

    let event_id = params.event_id;

    Html(format!("event_id: {event_id}"))
}

async fn menu_add(Query(params): Query<MenuAddParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler menu_add - {params:?}", "HANDLER");

    let name = params.name;
    let vendor_id = params.vendor_id;

    Html(format!("name: {name}\nvendor_id: {vendor_id}"))
}

async fn menu_remove(Query(params): Query<MenuRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler menu_remove - {params:?}", "HANDLER");

    let menu_id = params.menu_id;

    Html(format!("menu_id: {menu_id}"))
}

async fn item_add(Query(params): Query<ItemAddParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler item_add - {params:?}", "HANDLER");

    let name = params.name;
    let description = params.description.as_deref().unwrap_or("");
    let price = params.price.as_deref().unwrap_or("");
    let picture = params.picture.as_deref().unwrap_or("");
    let vendor_id = params.vendor_id;

    Html(format!("name: {name}\ndescription: {description}\nprice: {price}\npicture: {picture}\nvendor_id: {vendor_id}"))
}

async fn item_remove(Query(params): Query<ItemRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler item_remove - {params:?}", "HANDLER");

    let item_id = params.item_id;

    Html(format!("item_id: {item_id}"))
}

// endregion:   -- Handlers

// region:      -- Data Structures

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Day {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ReoccurancePattern {
    OneTime,
    Daily,
    Weekly { days: Vec<Day>, spacing: u32 },
    Monthly { day_of_month: u32, spacing: u32 },
    Yearly { month: Month, day_of_month: u32 },
}

impl fmt::Display for ReoccurancePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
struct VendorAddParams {
    name: String,
    auth_token: String,
    description: Option<String>,
    vendor_type: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    website: Option<String>,
    photo: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VendorRemoveParams {
    vendor_id: String,
}

#[derive(Debug, Deserialize)]
struct EventAddParams {
    datetime: String,
    location: String,
    repetition: String,
    vendor_id: String,
}

#[derive(Debug, Deserialize)]
struct EventRemoveParams {
    event_id: String,
}

#[derive(Debug, Deserialize)]
struct MenuAddParams {
    name: String,
    vendor_id: String,
}

#[derive(Debug, Deserialize)]
struct MenuRemoveParams {
    menu_id: String,
}

#[derive(Debug, Deserialize)]
struct ItemAddParams {
    name: String,
    description: Option<String>,
    price: Option<String>,
    picture: Option<String>,
    vendor_id: String,
}

#[derive(Debug, Deserialize)]
struct ItemRemoveParams {
    item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vendor {
    pub name: Cow<'static, str>,
    pub auth_token: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub vendor_type: Cow<'static, str>,
    pub email: Cow<'static, str>,
    pub phone_number: Cow<'static, str>,
    pub website: Cow<'static, str>,
    pub events: Vec<Event>,
    pub menu: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub name: Cow<'static, str>,
    pub datetime: Cow<'static, str>,
    pub location: Cow<'static, str>,
    pub menu: Option<Menu>,
    pub repeat_schedule: Cow<'static, str>,
    pub repeat_end: Cow<'static, str>,
    pub vendor: Option<Vendor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    pub name: Cow<'static, str>,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub price: u32,
    pub ingredients: Cow<'static, str>,
    pub picture: Cow<'static, str>,
    pub vendor: Vendor,
}

// endregion:   -- Data Structures
