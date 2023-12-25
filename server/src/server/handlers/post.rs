use std::collections::HashMap;

use crate::database::models::{Event, Item, Menu, Record, Vendor};
use crate::server::state;
use axum::extract::{Json as ExtractJson, Path, State};
use axum::response::IntoResponse;
use axum::response::Json;

// TODO: Test for bugs
pub async fn post_vendor(
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_add - {json:?}", "HANDLER");

    let vendor = Vendor::from(json);

    let record_option_result: Result<Option<Record>, surrealdb::Error> = state
        .db
        .create(("vendors", vendor.uuid.to_string()))
        .content(vendor)
        .await;
    let record_option = match record_option_result {
        Ok(record_option) => record_option,
        Err(err) => {
            println!("Failed to add vendor to database: {err:?}");
            return Json::default();
        }
    };
    let record = match record_option {
        Some(record) => record,
        None => return Json::default(),
    };

    Json(record)
}

// TODO: Test for bugs
pub async fn post_event(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!(
        "->> {:<12} - handler post_event - {params:?} - {json:?}",
        "HANDLER"
    );

    if let Some(vendor_id) = params.get("vendor_id") {
        let event = Event::from(json).with_vendor(vendor_id.into());

        let record_option_result: Result<Option<Record>, surrealdb::Error> = state
            .db
            .create(("events", event.uuid.to_string()))
            .content(event)
            .await;

        let record_option = match record_option_result {
            Ok(record_option) => record_option,
            Err(err) => {
                println!("Failed to add event to database: {err:?}");
                return Json::default();
            }
        };

        let record = match record_option {
            Some(record) => record,
            None => return Json::default(),
        };

        return Json(record);
    }
    Json::default()
}

// TODO: Test for bugs
pub async fn post_menu(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!(
        "->> {:<12} - handler menu_add - {params:?} - {json:?}",
        "HANDLER"
    );

    if let Some(vendor_id) = params.get("vendor_id") {
        let menu = Menu::from(json).with_vendor(vendor_id.into());

        let record_option_result: Result<Option<Record>, surrealdb::Error> = state
            .db
            .create(("menus", menu.uuid.to_string()))
            .content(menu)
            .await;

        let record_option = match record_option_result {
            Ok(record_option) => record_option,
            Err(err) => {
                println!("Failed to add menu to database: {err:?}");
                return Json::default();
            }
        };

        let record = match record_option {
            Some(record) => record,
            None => return Json::default(),
        };

        return Json(record);
    }
    Json::default()
}

// TODO: Test for bugs
pub async fn post_item(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler item_add - {json:?}", "HANDLER");
    if let Some(vendor_id) = params.get("vendor_id") {
        let item = Item::from(json).with_vendor(vendor_id.into());

        let record_option_result: Result<Option<Record>, surrealdb::Error> = state
            .db
            .create(("items", item.uuid.to_string()))
            .content(item)
            .await;

        let record_option = match record_option_result {
            Ok(record_option) => record_option,
            Err(err) => {
                println!("Failed to add item to database: {err:?}");
                return Json::default();
            }
        };

        let record = match record_option {
            Some(record) => record,
            None => return Json::default(),
        };

        return Json(record);
    }
    Json::default()
}
