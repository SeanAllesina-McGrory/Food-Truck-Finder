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
    if let Some(vendor_id) = params.vendor_id {
        let vendor_result: Result<Option<Vendor>, surrealdb::Error> =
            state.db.select(("vendors", vendor_id)).await;
        let vendor_option = match vendor_result {
            Ok(vendor_option) => vendor_option,
            Err(_) => return Json::default(),
        };
        let vendor = match vendor_option {
            Some(vendor) => vendor,
            None => return Json::default(),
        };
        return Json(vec![vendor]);
    } else if let Some(event_id) = params.event_id {
        let vendors_vec_result: Result<Vec<Vendor>, surrealdb::Error> =
            state.db.select("vendors").await;
        let vendors_vec: Vec<Vendor> = match vendors_vec_result {
            Ok(vendors_vec) => vendors_vec,
            Err(err) => {
                println!("{:?}", err);
                return Json::default();
            }
        };
        //println!("{:?}", vendors_vec);
        let vendors_vec: Vec<Vendor> = vendors_vec
            .iter()
            .map(|v| v.clone())
            .filter(|vendor| {
                println!("{:?}", vendor.events);
                let es = &vendor
                    .events
                    .iter()
                    .map(|e| e.clone())
                    .filter(|event| {
                        println!("{}", event.uuid);
                        return event.uuid == event_id;
                    })
                    .collect::<Vec<Event>>();
                println!("{:?}", es);
                es.len() != 0
            })
            .collect::<Vec<Vendor>>();
        println!("{:?}", vendors_vec);
        return Json(vendors_vec);
    } else if let Some(menu_id) = params.menu_id {
        todo!();
    } else if let Some(item_id) = params.item_id {
        todo!();
    }
    Json::default()
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
    if let Some(event_id) = params.event_id {
        let event_result: Result<Option<Event>, surrealdb::Error> =
            state.db.select(("events", event_id)).await;
        let event_option = match event_result {
            Ok(event_option) => event_option,
            Err(err) => return Json::default(),
        };
        let event = match event_option {
            Some(event) => event,
            None => return Json::default(),
        };
        return Json(vec![event]);
    } else if let Some(vendor_id) = params.vendor_id {
        let events_vec_result: Result<Vec<Event>, surrealdb::Error> =
            state.db.select("events").await;
        let events_vec = match events_vec_result {
            Ok(events_vec) => events_vec,
            Err(_) => Vec::new(),
        };
        let events_vec: Vec<Event> = events_vec
            .iter()
            .map(|e| e.clone())
            .filter(|event| match &event.vendor {
                Some(vendor) => vendor.uuid == vendor_id,
                None => false,
            })
            .collect();
        return Json(events_vec);
    }
    Json::default()
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
        .content(Event::new(
            String::from("Now"),
            String::from("There"),
            vendor,
        ))
        .await
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

    let menu: Vec<Menu> = match db.create("menu").content(Menu::new(name, vendor)).await {
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

    let item: Vec<Item> = match db.create("item").content(Item::new(name, vendor)).await {
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
