use std::collections::HashMap;

use crate::database::models::{Event, Item, Menu, Vendor};
use crate::server::state;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::response::Json;

// TODO: Test for bugs
pub async fn delete_vendor(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_remove - {params:?}", "HANDLER");

    if let Some(vendor_id) = params.get("vendor_id") {
        let db_resp = state.db.delete(("vendors", vendor_id)).await;
        let vendor: Vendor = match db_resp {
            Ok(vendor_option) => match vendor_option {
                Some(vendor) => vendor,
                None => {
                    return Json::default();
                }
            },
            Err(err) => {
                println!("Failed in result match\n{err:?}");
                return Json::default();
            }
        };
        return Json(vendor);
    }
    Json::default()
}

// TODO: Test for bugs
pub async fn delete_event(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler event_remove - {params:?}", "HANDLER");
    if let Some(event_id) = params.get("event_id") {
        let db_resp = state.db.delete(("events", event_id)).await;

        let event: Event = match db_resp {
            Ok(event_option) => match event_option {
                Some(event) => event,
                None => return Json::default(),
            },
            Err(err) => {
                println!("Failed to delete event: {err:?}");
                return Json::default();
            }
        };

        return Json(event);
    }
    Json::default()
}

// TODO: Test for bugs
pub async fn delete_menu(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler menu_remove - {params:?}", "HANDLER");
    if let Some(menu_id) = params.get("menu_id") {
        let db_resp = state.db.delete(("menus", menu_id)).await;

        let menu: Menu = match db_resp {
            Ok(menu_option) => match menu_option {
                Some(menu) => menu,
                None => return Json::default(),
            },
            Err(err) => {
                println!("Failed to delete menu: {err:?}");
                return Json::default();
            }
        };

        return Json(menu);
    }
    Json::default()
}

// TODO: Test for bugs
pub async fn delete_item(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler item_remove - {params:?}", "HANDLER");

    if let Some(item_id) = params.get("item_id") {
        let db_resp = state.db.delete(("items", item_id)).await;

        let item: Item = match db_resp {
            Ok(item_option) => match item_option {
                Some(item) => item,
                None => return Json::default(),
            },
            Err(err) => {
                println!("Failed to delete item: {err:?}");
                return Json::default();
            }
        };

        return Json(item);
    }
    Json::default()
}
