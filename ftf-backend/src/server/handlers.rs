use crate::database::models;
use crate::database::models::{Event, Item, Menu, Vendor};
use crate::server::state;
use axum::extract::{Json as ExtractJson, Query, State};
use axum::response::IntoResponse;
use axum::response::{Html, Json};
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

// TODO: Bug test
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
                println!("Failed in result match: Line 40\n{:?}", err);
                return Json::default();
            }
        };
        let vendors_vec: Vec<Vendor> = vendors_vec
            .iter()
            .map(|v| v.clone())
            .filter(|vendor| {
                let es = &vendor
                    .events
                    .iter()
                    .map(|e| e.clone())
                    .filter(|event| {
                        return event.id.to_string() == event_id;
                    })
                    .map(|event_thing| event_thing.id.to_string())
                    .collect::<Vec<String>>();
                es.len() != 0
            })
            .collect::<Vec<Vendor>>();
        return Json(vendors_vec);
    } else if let Some(menu_id) = params.menu_id {
        let vendors_vec_result: Result<Vec<Vendor>, surrealdb::Error> =
            state.db.select("vendors").await;
        let vendors_vec: Vec<Vendor> = match vendors_vec_result {
            Ok(vendors_vec) => vendors_vec,
            Err(err) => {
                println!("Failed in result match: Line 67\n{:?}", err);
                return Json::default();
            }
        };
        let vendors_vec: Vec<Vendor> = vendors_vec
            .iter()
            .map(|v| v.clone())
            .filter(|vendor| {
                let menus = &vendor
                    .menus
                    .iter()
                    .map(|m| m.clone())
                    .filter(|menu| {
                        return menu.id.to_string() == menu_id;
                    })
                    .map(|menu_thing| menu_thing.id.to_string())
                    .collect::<Vec<String>>();
                menus.len() != 0
            })
            .collect::<Vec<Vendor>>();
        return Json(vendors_vec);
    } else if let Some(item_id) = params.item_id {
        let items_vec_result: Result<Vec<Item>, surrealdb::Error> = state.db.select("items").await;
        let items_vec: Vec<Item> = match items_vec_result {
            Ok(items_vec) => items_vec,
            Err(err) => {
                println!("Failed in result match: Line 93\n{:?}", err);
                return Json::default();
            }
        };
        let vendors: Vec<String> = items_vec
            .iter()
            .map(|i| i.clone())
            .filter(|item| item.uuid == item_id)
            .map(|item| match item.vendor {
                Some(vendor) => vendor.id.to_string(),
                None => "".into(),
            })
            .collect::<Vec<String>>();
        match vendors.len() {
            0 => (),
            1 => {
                let vendor_option_result: Result<Option<Vendor>, surrealdb::Error> =
                    state.db.select(("vendors", &vendors[0])).await;
                let vendor_option = match vendor_option_result {
                    Ok(vendor_option) => vendor_option,
                    Err(err) => {
                        println!("Failed in result match: Line 116\n{:?}", err);
                        return Json::default();
                    }
                };
                let _: Vendor = match vendor_option {
                    Some(vendor) => return Json(vec![vendor]),
                    None => return Json::default(),
                };
            }
            _ => {
                println!(
                    "ERROR: UNIQUE ITEM ID COROSPONDED TO MULTIPLE ITEMS\nLogging Descrepency"
                );
                // TODO: adding logging
            }
        }
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

// TODO: Bug test
pub async fn vendor_add(
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

// WARNING: There is essentially no situation where you should need to use this with any amount of
// regularity. Keeping here to maintain parity between tables, but should be removed before pro
// TODO: Bug test
pub async fn vendor_remove(
    State(state): State<state::AppState>,
    Json(json): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    println!("->> {:<12} - handler vendor_remove - {json:?}", "HANDLER");

    let vendor_id_option: Option<&Value> = json.get("vendor_id");
    let vendor_id = match vendor_id_option {
        Some(vendor_id) => vendor_id,
        None => return Json::default(),
    };

    let db_resp = state
        .db
        .delete(("vendors", vendor_id.as_str().unwrap()))
        .await;
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

// TODO: Bug test
pub async fn event_get(
    Query(params): Query<models::EventGetParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    if let Some(event_id) = params.event_id {
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
                Some(vendor) => vendor.id.to_string() == vendor_id,
                None => false,
            })
            .collect();
        return Json(events_vec);
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

// FIX: Recode to align with Thing based db linking and JSON returns
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

// FIX: Recode to align with Thing based db linking and JSON returns
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

// TODO: Bug test
pub async fn menu_get(
    Query(params): Query<models::MenuGetParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    if let Some(menu_id) = params.menu_id {
        let menu_option_result: Result<Option<Menu>, surrealdb::Error> =
            state.db.select(("menus", menu_id)).await;
        let menu_option = match menu_option_result {
            Ok(menu_option) => menu_option,
            Err(err) => {
                println!("Failed in result match: Line 293\n{:?}", err);
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
    } else if let Some(vendor_id) = params.vendor_id {
        let menu_vec_result: Result<Vec<Menu>, surrealdb::Error> = state.db.select("menus").await;
        let menu_vec = match menu_vec_result {
            Ok(menu_vec) => menu_vec,
            Err(err) => {
                println!("Failed in result match: Line 310\n{:?}", err);
                return Json::default();
            }
        };
        let menu_vec = menu_vec
            .iter()
            .map(|m| m.clone())
            .filter(|menu| match &menu.vendor {
                Some(vendor) => vendor.id.to_string() == vendor_id,
                None => false,
            })
            .collect();
        return Json(menu_vec);
    } else if let Some(event_id) = params.event_id {
        let event_option_result: Result<Option<Event>, surrealdb::Error> =
            state.db.select(("events", event_id)).await;
        let event_option = match event_option_result {
            Ok(event_option) => event_option,
            Err(err) => {
                println!("Failed in result match: Line 328\n{:?}", err);
                return Json::default();
            }
        };
        let event = match event_option {
            Some(event) => event,
            None => {
                return Json::default();
            }
        };
        let menu_id = match event.menu {
            Some(menu) => menu.id,
            None => {
                return Json::default();
            }
        };
        let menu_option_result: Result<Option<Menu>, surrealdb::Error> =
            state.db.select(("menus", menu_id)).await;
        let menu_option = match menu_option_result {
            Ok(menu_option) => menu_option,
            Err(err) => {
                println!("Failed in result match: Line 349\n{:?}", err);
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
    }
    let menu_vec_result: Result<Vec<Menu>, surrealdb::Error> = state.db.select("menus").await;
    let menu_vec = match menu_vec_result {
        Ok(menu_vec) => menu_vec,
        Err(err) => {
            println!("{:?}", err);
            return Json::default();
        }
    };
    Json(menu_vec)
}

// FIX: Recode to align with Thing based db linking and JSON returns
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

// FIX: Recode to align with Thing based db linking and JSON returns
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

// TODO: Bug test
pub async fn item_get(
    Query(params): Query<models::ItemGetParams>,
    State(state): State<state::AppState>,
) -> impl IntoResponse {
    if let Some(item_id) = params.item_id {
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
    } else if let Some(vendor_id) = params.vendor_id {
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
                Some(vendor) => vendor.id.to_string() == vendor_id,
                None => false,
            })
            .collect();
        return Json(item_vec);
    } else if let Some(menu_id) = params.menu_id {
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
    let item_vec_result: Result<Vec<Item>, surrealdb::Error> = state.db.select("items").await;
    let item_vec = match item_vec_result {
        Ok(item_vec) => item_vec,
        Err(err) => {
            println!("{:?}", err);
            return Json::default();
        }
    };
    Json(item_vec)
}

// FIX: Recode to align with Thing based db linking and JSON returns
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

// FIX: Recode to align with Thing based db linking and JSON returns
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
