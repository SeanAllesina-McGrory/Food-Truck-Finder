use std::collections::HashMap;

use crate::database::models::{Event, Item, Menu, Vendor};
use crate::server::state;
use axum::extract::{Json as ExtractJson, Path, State};
use axum::response::IntoResponse;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

impl Default for Record {
    fn default() -> Self {
        Record {
            id: Thing {
                tb: "".into(),
                id: "".into(),
            },
        }
    }
}

// TODO: Test for bugs
pub async fn get_vendors(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    if let Some(vendor_id) = params.get("vendor_id") {
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
    }
    let vendor_vec_result: Result<Vec<Vendor>, surrealdb::Error> = state.db.select("vendors").await;
    let vendor_vec = match vendor_vec_result {
        Ok(vendor_vec) => vendor_vec,
        Err(err) => {
            println!("Failed in result match\n{:?}", err);
            return Json::default();
        }
    };
    Json(vendor_vec)
}

// TODO: Test for bugs
pub async fn get_events(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> HANDLER - Events GET - {params:<60?}");
    if let Some(vendor_id) = params.get("vendor_id") {
        let events_vec_result: Result<Vec<Event>, surrealdb::Error> =
            state.db.select("events").await;
        let events_vec = match events_vec_result {
            Ok(events_vec) => events_vec,
            Err(err) => {
                println!("Failed in result match\n{err:?}");
                return Json::default();
            }
        };
        let events_vec: Vec<Event> = events_vec
            .iter()
            .map(|e| e.clone())
            .filter(|event| match &event.vendor {
                Some(vendor) => vendor.id.to_string() == vendor_id.clone(),
                None => false,
            })
            .collect();
        return Json(events_vec);
    } else if let Some(event_id) = params.get("event_id") {
        let event_result: Result<Option<Event>, surrealdb::Error> =
            state.db.select(("events", event_id)).await;
        let event_option = match event_result {
            Ok(event_option) => event_option,
            Err(err) => {
                println!("Failed in result match\n{:?}", err);
                return Json::default();
            }
        };
        let event = match event_option {
            Some(event) => event,
            None => return Json::default(),
        };
        return Json(vec![event]);
    }
    let event_vec_result: Result<Vec<Event>, surrealdb::Error> = state.db.select("events").await;
    let event_vec = match event_vec_result {
        Ok(event_vec) => event_vec,
        Err(err) => {
            println!("{:?}", err);
            return Json::default();
        }
    };
    Json(event_vec)
}

// TODO: Test for bugs
pub async fn get_menus(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    if let Some(menu_id) = params.get("menu_id") {
        let menu_option_result: Result<Option<Menu>, surrealdb::Error> =
            state.db.select(("menus", menu_id)).await;
        let menu_option = match menu_option_result {
            Ok(menu_option) => menu_option,
            Err(err) => {
                println!("failed in result match: line 293\n{:?}", err);
                return Json::default();
            }
        };
        let menu = match menu_option {
            Some(menu) => menu,
            None => {
                return Json::default();
            }
        };
        return Json(vec![menu]);
    } else if let Some(vendor_id) = params.get("vendor_id") {
        let menu_vec_result: Result<Vec<Menu>, surrealdb::Error> = state.db.select("menus").await;
        let menu_vec = match menu_vec_result {
            Ok(menu_vec) => menu_vec,
            Err(err) => {
                println!("failed in result match: line 310\n{:?}", err);
                return Json::default();
            }
        };
        let menu_vec = menu_vec
            .iter()
            .map(|m| m.clone())
            .filter(|menu| match &menu.vendor {
                Some(vendor) => vendor.id.to_string() == vendor_id.to_owned(),
                None => false,
            })
            .collect();
        return Json(menu_vec);
    }
    Json(vec![])
}

// TODO: Test for bugs
pub async fn get_items(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    if let Some(item_id) = params.get("item_id") {
        let item_option_result: Result<Option<Item>, surrealdb::Error> =
            state.db.select(("items", item_id)).await;
        let item_option = match item_option_result {
            Ok(item_option) => item_option,
            Err(err) => {
                println!("Failed in result match\n{:?}", err);
                return Json::default();
            }
        };
        let item = match item_option {
            Some(item) => item,
            None => {
                return Json::default();
            }
        };
        return Json(vec![item]);
    } else if let Some(vendor_id) = params.get("vendor_id") {
        let item_vec_result: Result<Vec<Item>, surrealdb::Error> = state.db.select("items").await;
        let item_vec = match item_vec_result {
            Ok(item_vec) => item_vec,
            Err(err) => {
                println!("Failed in result match\n{:?}", err);
                return Json::default();
            }
        };
        let item_vec = item_vec
            .iter()
            .map(|i| i.clone())
            .filter(|item| match &item.vendor {
                Some(vendor) => vendor.id.to_string() == vendor_id.to_owned(),
                None => false,
            })
            .collect();
        return Json(item_vec);
    } else if let Some(menu_id) = params.get("menu_id") {
        let menu_option_result: Result<Option<Menu>, surrealdb::Error> =
            state.db.select(("menus", menu_id)).await;
        let menu_option = match menu_option_result {
            Ok(menu_option) => menu_option,
            Err(err) => {
                println!("Failed in result match\n{:?}", err);
                return Json::default();
            }
        };
        let menu = match menu_option {
            Some(menu) => menu,
            None => {
                return Json::default();
            }
        };

        let menu_items: Vec<String> = menu.items.iter().map(|item| item.id.to_string()).collect();

        let items_vec_result: Result<Vec<Item>, surrealdb::Error> = state.db.select("items").await;

        let items_vec = match items_vec_result {
            Ok(items_vec) => items_vec,
            Err(err) => {
                println!("Failed in result match\n{:?}", err);
                return Json::default();
            }
        };

        let items_vec = items_vec
            .iter()
            .map(|i| i.clone())
            .filter(|item| menu_items.contains(&item.uuid.to_string()))
            .collect();

        return Json(items_vec);
    }
    Json(vec![])
}

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
    println!("->> {:<12} - handler vendor_add - {json:?}", "HANDLER");

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
    println!("->> {:<12} - handler menu_add - {json:?}", "HANDLER");

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
