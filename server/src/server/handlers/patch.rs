use std::collections::HashMap;

use crate::database::models::{Item, Menu, Vendor};
use crate::server::state;
use axum::extract::{Json as ExtractJson, Path, State};
use axum::response::IntoResponse;
use axum::response::Json;
use serde_json::Value;
use surrealdb::opt::PatchOp;

pub async fn patch_vendor(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    if let Some(vendor_id) = params.get("vendor_id") {
        let mut patch_pairs: Vec<(&str, &Value)> = Vec::new();

        if let Some(name) = json.get("name") {
            patch_pairs.push(("/name", name));
        }
        if let Some(description) = json.get("description") {
            patch_pairs.push(("/description", description));
        }
        if let Some(vendor_type) = json.get("vendor_type") {
            patch_pairs.push(("/vendor_type", vendor_type));
        }
        if let Some(email) = json.get("email") {
            patch_pairs.push(("/email", email));
        }
        if let Some(phone_number) = json.get("phone_number") {
            patch_pairs.push(("/phone_number", phone_number));
        }
        if let Some(website) = json.get("website") {
            patch_pairs.push(("/website", website));
        }

        let mut vendor_option_result: Result<Option<Vendor>, surrealdb::Error> = Ok(None);

        for patch_pair in patch_pairs.iter() {
            vendor_option_result = state
                .db
                .update(("vendors", vendor_id))
                .patch(PatchOp::replace(patch_pair.0, patch_pair.1))
                .await;
        }

        let vendor_option = match vendor_option_result {
            Ok(vendor_option) => vendor_option,
            Err(err) => {
                println!("Failed in result match\n{err:?}");
                return Json::default();
            }
        };

        let vendor = match vendor_option {
            Some(vendor) => vendor,
            None => return Json::default(),
        };

        return Json(vendor);
    }
    Json::default()
}

pub async fn patch_event(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    if let Some(event_id) = params.get("event_id") {
        let mut patch_pairs: Vec<(&str, &Value)> = Vec::new();
        if let Some(name) = json.get("name") {
            patch_pairs.push(("/name", name));
        }
        if let Some(datetime) = json.get("datetime") {
            patch_pairs.push(("/datetime", datetime));
        }
        if let Some(location) = json.get("location") {
            patch_pairs.push(("/location", location));
        }
        if let Some(menu) = json.get("menu") {
            patch_pairs.push(("/menu", menu));
        }
        if let Some(repeat_schedule) = json.get("repeat_schedule") {
            patch_pairs.push(("/repeat_schedule", repeat_schedule));
        }
        if let Some(repeat_end) = json.get("repeat_end") {
            patch_pairs.push(("/repeat_end", repeat_end));
        }

        let mut event_option_result: Result<Option<Menu>, surrealdb::Error> = Ok(None);

        for patch_pair in patch_pairs.iter() {
            event_option_result = state
                .db
                .update(("events", event_id))
                .patch(PatchOp::replace(patch_pair.0, patch_pair.1))
                .await;
        }

        let event_option = match event_option_result {
            Ok(event_option) => event_option,
            Err(err) => {
                println!("Failed in result match\n{err:?}");
                return Json::default();
            }
        };

        let event = match event_option {
            Some(event) => event,
            None => return Json::default(),
        };

        return Json(event);
    }
    Json::default()
}

pub async fn patch_menu(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    if let Some(menu_id) = params.get("menu_id") {
        if let Some(name) = json.get("name") {
            let menu_option_result: Result<Option<Menu>, surrealdb::Error> = state
                .db
                .update(("menus", menu_id))
                .patch(PatchOp::replace("/name", name))
                .await;
            let menu_option = match menu_option_result {
                Ok(menu_option) => menu_option,
                Err(err) => {
                    println!("Failed in result match\n{err:?}");
                    return Json::default();
                }
            };

            let menu = match menu_option {
                Some(menu) => menu,
                None => return Json::default(),
            };

            return Json(menu);
        }
    }
    Json::default()
}

pub async fn patch_item(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    if let Some(item_id) = params.get("item_id") {
        let mut patch_pairs: Vec<(&str, &Value)> = Vec::new();
        if let Some(name) = json.get("name") {
            patch_pairs.push(("/name", name));
        }
        if let Some(description) = json.get("description") {
            patch_pairs.push(("/description", description));
        }
        if let Some(price) = json.get("price") {
            patch_pairs.push(("/price", price));
        }
        if let Some(picture) = json.get("picture") {
            patch_pairs.push(("/picture", picture));
        }

        let mut item_option_result: Result<Option<Item>, surrealdb::Error> = Ok(None);

        for patch_pair in patch_pairs.iter() {
            item_option_result = state
                .db
                .update(("items", item_id))
                .patch(PatchOp::replace(patch_pair.0, patch_pair.1))
                .await;
        }

        let item_option = match item_option_result {
            Ok(item_option) => item_option,
            Err(err) => {
                println!("Failed in result match Test\n{err:?}");
                return Json::default();
            }
        };

        let item = match item_option {
            Some(item) => item,
            None => return Json::default(),
        };

        return Json(item);
    }
    Json::default()
}
