use crate::database::models;
use crate::database::models::{Event, Item, Menu, Vendor};
use crate::server::state;
use axum::extract::{Query, State};
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Json;
use std::borrow::Cow;

pub async fn vendor_get(
    Query(params): Query<models::VendorGetParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("--> {:<12} - handler vendor_get - {params:?}", "HANDLER");

    let db = state.db;
    match params.vendor_id {
        Some(vendor_id) => return Json(format!("{:?}", vendor_id)),
        None => {
            let vendor: Vec<Vendor> = match db.select("vendor").await {
                Ok(vendor) => vendor,
                Err(err) => {
                    println!("{:?}", err);
                    return Json(format!("{:?}", err));
                }
            };
            Json(format!("{:?}", vendor))
        }
    }
}

pub async fn vendor_add(
    Query(params): Query<models::VendorAddParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_add - {params:?}", "HANDLER");

    let db = state.db;

    let vendor: Vec<Vendor> = match db
        .create("vendor")
        .content(Vendor::new(params.name, params.auth_token))
        .await
    {
        Ok(res) => res,
        Err(err) => return Html(format!("{:?}", err)),
    };

    Html(format!("{:?}", vendor))
}

pub async fn vendor_remove(
    Query(params): Query<models::VendorRemoveParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_remove - {params:?}", "HANDLER");

    let db = state.db;
    let vendor_id = params.vendor_id;

    let vendor: Vec<Vendor> = match db.delete(("vendor", vendor_id.clone())).await {
        Ok(vendor_option) => match vendor_option {
            Some(vendor) => vendor,
            None => return Html(format!("[]")),
        },
        Err(err) => return Html(format!("{:?}", err)),
    };

    Html(format!("{:?}", vendor))
}

pub async fn event_get(
    Query(params): Query<models::EventGetParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    Json(format!("{}", String::from("Hello, Cruel World!")))
}

pub async fn event_add(
    Query(params): Query<models::EventAddParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler event_add - {params:?}", "HANDLER");

    let db = state.db;
    
    let vendor = match db.select(("vendor", params.vendor_id)).await {
        Ok(option_vendor) => match option_vendor {
            Some(vendor) => vendor,
            None => return Html(format!("Vendor not found")),
        },
        Err(err) => return Html(format!("{:?}", err)),
    };

    let datetime = params.datetime;
    let location = params.location;
    let repetition = params.repetition;

    let event: Vec<Event> = match db
        .create("event")
        .content(Event::new(String::from("Now"), String::from("There"), vendor)).await
    {
        Ok(event) => event,
        Err(err) => return Html(format!("{:?}", err)),
    };
    Html(format!("{:?}", event))
}

pub async fn event_remove(
    Query(params): Query<models::EventRemoveParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler event_remove - {params:?}", "HANDLER");
    let db = state.db;
    let event_id = params.event_id;
    let event: Vec<Event> = match db.delete(("vendor", event_id.clone())).await {
        Ok(event_option) => match event_option {
            Some(event) => event,
            None => return Html(format!("[]")),
        },
        Err(err) => return Html(format!("{:?}", err)),
    };
    Html(format!("{:?}", event))
}

pub async fn menu_get(
    Query(params): Query<models::MenuGetParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    Json(format!("{}", String::from("Hello, Cruel World!")))
}

pub async fn menu_add(
    Query(params): Query<models::MenuAddParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler menu_add - {params:?}", "HANDLER");

    let db = state.db;

    let vendor = match db.select(("vendor", params.vendor_id)).await {
        Ok(option_vendor) => match option_vendor {
            Some(vendor) => vendor,
            None => return Html(format!("Vendor not found")),
        },
        Err(err) => return Html(format!("{:?}", err)),
    };
    
    let name = params.name;
    let items: String = params.items.unwrap_or(String::from("")).into();

    let menu: Vec<Menu> = match db
        .create("menu")
        .content(Menu::new(name, vendor))
        .await
    {
        Ok(event) => event,
        Err(err) => return Html(format!("{:?}", err)),
    };
    Html(format!("{:?}", menu))
}

pub async fn menu_remove(
    Query(params): Query<models::MenuRemoveParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler menu_remove - {params:?}", "HANDLER");

    let db = state.db;
    let menu_id = params.menu_id;

    let menu: Vec<Menu> = match db.delete(("menu", menu_id)).await {
        Ok(menu_option) => match menu_option {
            Some(menu) => menu,
            None => return Html(format!("[]")),
        },
        Err(err) => return Html(format!("{:?}", err)),
    };

    Html(format!("{:?}", menu))
}

pub async fn item_get(
    Query(params): Query<models::ItemGetParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    Json(format!("{}", String::from("Hello, Cruel World!")))
}

pub async fn item_add(
    Query(params): Query<models::ItemAddParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler item_add - {params:?}", "HANDLER");

    let db = state.db;

    let vendor = match db.select(("vendor", params.vendor_id)).await {
        Ok(option_vendor) => match option_vendor {
            Some(vendor) => vendor,
            None => return Html(format!("Vendor not found")),
        },
        Err(err) => return Html(format!("{:?}", err)),
    };

    let name = params.name;
    let description: String = match params.description {
        Some(string) => string,
        None => String::from(""),
    }
    .into();
    let price: String = match params.price {
        Some(string) => string,
        None => String::from(""),
    }
    .into();
    let picture: String = match params.picture {
        Some(string) => string,
        None => String::from(""),
    }
    .into();

    let item: Vec<Item> = match db
        .create("item")
        .content(Item::new(name, vendor))
        .await
    {
        Ok(item) => item,
        Err(err) => return Html(format!("{:?}", err)),
    };

    Html(format!("{:?}", item))
}

pub async fn item_remove(
    Query(params): Query<models::ItemRemoveParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler item_remove - {params:?}", "HANDLER");

    let db = state.db;

    let item_id = params.item_id;

    let item: Vec<Item> = match db.delete(("item", item_id)).await {
        Ok(item_option) => match item_option {
            Some(item) => item,
            None => return Html(format!("[]")),
        },
        Err(err) => return Html(format!("{:?}", err)),
    };

    Html(format!("{:?}", item))
}
