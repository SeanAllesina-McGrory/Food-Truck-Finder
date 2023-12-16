use std::collections::HashMap;

use crate::database::models;
use crate::database::models::{Event, Item, Menu, Vendor};
use crate::server::state;
use axum::extract::{Json as ExtractJson, Path, Query, State};
use axum::response::IntoResponse;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
            println!("Failed in result match: Line 136\n{:?}", err);
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
                println!("Failed in result match: Line 71\n{err:?}");
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
                println!("Failed in result match: Line 196\n{:?}", err);
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
                println!("Failed in result match: Line 432\n{:?}", err);
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
                println!("Failed in result match: Line 448\n{:?}", err);
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
                println!("Failed in result match: Line 466\n{:?}", err);
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
                println!("Failed in result match: Line 484\n{:?}", err);
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
    Json("Updated Vendor")
}

pub async fn patch_event(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    Json("Updated Event")
}

pub async fn patch_menu(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    Json("Updated Menu")
}

pub async fn patch_item(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    Json("Updated Item")
}

pub async fn post_vendor(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_add - {json:?}", "HANDLER");

    let record_json = Vendor::from(json);

    let record_option_result: Result<Option<Record>, surrealdb::Error> = state
        .db
        .create(("vendors", record_json.uuid.to_string()))
        .content(record_json)
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

pub async fn post_event(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_add - {json:?}", "HANDLER");

    let record_json = Vendor::from(json);

    let record_option_result: Result<Option<Record>, surrealdb::Error> = state
        .db
        .create(("vendors", record_json.uuid.to_string()))
        .content(record_json)
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

pub async fn post_menu(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler menu_add - {json:?}", "HANDLER");

    let menu = Menu::from(json);

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
    Json(record)
}

pub async fn post_item(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler item_add - {json:?}", "HANDLER");

    let item = Item::from(json);

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

    Json(record)
}

pub async fn delete_vendor(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_remove - {params:?}", "HANDLER");

    let vendor_id_option: Option<&String> = params.get("vendor_id");
    let vendor_id = match vendor_id_option {
        Some(vendor_id) => vendor_id,
        None => return Json::default(),
    };

    let db_resp = state.db.delete(("vendors", vendor_id)).await;
    let vendor: Vendor = match db_resp {
        Ok(vendor_option) => match vendor_option {
            Some(vendor) => vendor,
            None => {
                return Json::default();
            }
        },
        Err(err) => {
            println!("Failed in result match: Line 209\n{err:?}");
            return Json::default();
        }
    };
    Json(vendor)
}

pub async fn delete_event(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler event_remove - {params:?}", "HANDLER");

    let event_id_option: Option<&String> = params.get("event_id");
    let event_id = match event_id_option {
        Some(event_id) => event_id,
        None => return Json::default(),
    };

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

    Json(event)
}

pub async fn delete_menu(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler menu_remove - {params:?}", "HANDLER");

    let menu_id_option: Option<&String> = params.get("menu_id");
    let menu_id = match menu_id_option {
        Some(menu_id) => menu_id,
        None => return Json::default(),
    };

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

    Json(menu)
}

pub async fn delete_item(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler item_remove - {params:?}", "HANDLER");

    let item_id_option: Option<&String> = params.get("item_id");
    let item_id = match item_id_option {
        Some(item_id) => item_id,
        None => return Json::default(),
    };

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

    Json(item)
}
