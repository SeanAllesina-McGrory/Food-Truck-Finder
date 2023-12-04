use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use chrono::prelude::*;
use geoutils::Location;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use tower_http::cors::{Any, CorsLayer};

pub fn make_app() -> Router {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .layer(cors)
        .route("/vendor/add", get(handler_vendor_add))
        .route("/vendor/remove", get(handler_vendor_remove))
        .route("/event/add", get(handler_event_add))
        .route("/event/remove", get(handler_event_remove))
        .route("/menu/add", get(handler_menu_add))
        .route("/menu/remove", get(handler_menu_remove))
        .route("/item/add", get(handler_item_add))
        .route("/item/remove", get(handler_item_remove));
    app
}

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
    pub description: Option<Cow<'static, str>>,
    pub vendor_type: Option<Cow<'static, str>>,
    pub email: Option<Cow<'static, str>>,
    pub phone_number: Option<Cow<'static, str>>,
    pub website: Option<Cow<'static, str>>,
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

// region:      -- Handlers

async fn handler_vendor_add(Query(params): Query<VendorAddParams>) -> impl IntoResponse {
   println!("->> {:<12} - handler_vendor_add - {params:?}", "HANDLER");

   let name = params.name;
   let auth_token = params.auth_token;
   let description = params.description.as_deref().unwrap_or("");
   let vendor_type = params.vendor_type.as_deref().unwrap_or("");
   let email = params.email.as_deref().unwrap_or("");
   let phone_number = params.phone_number.as_deref().unwrap_or("");
   let website = params.website.as_deref().unwrap_or("");
   let photo = params.photo.as_deref().unwrap_or("");

    Html(format!("name: {name}\nauth_token: {auth_token}\ndescription: {description}\nvendor_type: {vendor_type}\nemail: {email}\nphone_number: {phone_number}\nwebsite: {website}\nphoto: {photo}"))
}

async fn handler_vendor_remove(Query(params): Query<VendorRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_vendor_remove - {params:?}", "HANDLER");

    let vendor_id = params.vendor_id;

    Html(format!("vendor_id: {vendor_id}"))
}

async fn handler_event_add(Query(params): Query<EventAddParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_event_add - {params:?}", "HANDLER");

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

    Html(format!("datetime: {0}\nlocation: {1}\nrepetition: {2}\nvendor_id: {3:?}", event.datetime, event.location, event.repeat_schedule, event.vendor))
}

async fn handler_event_remove(Query(params): Query<EventRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_event_remove - {params:?}", "HANDLER");

    let event_id = params.event_id;

    Html(format!("event_id: {event_id}"))
}

async fn handler_menu_add(Query(params): Query<MenuAddParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_menu_add - {params:?}", "HANDLER");

    let name = params.name;
    let vendor_id = params.vendor_id;

    Html(format!("name: {name}\nvendor_id: {vendor_id}"))
}

async fn handler_menu_remove(Query(params): Query<MenuRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_menu_remove - {params:?}", "HANDLER");

    let menu_id = params.menu_id;

    Html(format!("menu_id: {menu_id}"))
}

async fn handler_item_add(Query(params): Query<ItemAddParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_item_add - {params:?}", "HANDLER");

    let name = params.name;
    let description = params.description.as_deref().unwrap_or("");
    let price = params.price.as_deref().unwrap_or("");
    let picture = params.picture.as_deref().unwrap_or("");
    let vendor_id = params.vendor_id;

    Html(format!("name: {name}\ndescription: {description}\nprice: {price}\npicture: {picture}\nvendor_id: {vendor_id}"))
}

async fn handler_item_remove(Query(params): Query<ItemRemoveParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_item_remove - {params:?}", "HANDLER");

    let item_id = params.item_id;

    Html(format!("item_id: {item_id}"))
}
// endregion:   -- Handlers
