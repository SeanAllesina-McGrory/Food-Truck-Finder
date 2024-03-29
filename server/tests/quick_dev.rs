#![allow(unused)]
#[path = "../src/database.rs"]
mod database;
#[path = "../src/server.rs"]
mod server;
#[path = "../src/utils.rs"]
mod utils;
use crate::database::models::{Event, Item, Menu, Record, ReoccurancePattern, Vendor};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::prelude::*;
use color_eyre::{eyre::anyhow, Result};
use colored::Colorize;
use csv::{ReaderBuilder, StringRecord};
use dotenv;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use geoutils::Location;
use serde::{Deserialize, Serialize};
use std::env;
use std::{borrow::Cow, collections::HashMap};
use std::{fs::File, io::Read};
use surrealdb::{engine::remote::ws::Client, sql::Thing};
use surrealdb::{sql::Id, Surreal};
use tokio::sync::futures;

#[derive(Debug, Deserialize)]
struct VendorRecord {
    name: String,
    phone: String,
    email: String,
    county: String,
}

impl VendorRecord {
    fn new(records: StringRecord) -> Self {
        Self {
            name: records.get(0).unwrap().to_string(),
            phone: records.get(1).unwrap_or("").to_string(),
            email: records.get(2).unwrap().to_string(),
            county: records.get(3).unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct EventRecord {
    name: String,
    datetime: String,
    location: String,
    end_date: String,
}

impl EventRecord {
    fn new(records: StringRecord) -> Self {
        Self {
            name: records.get(0).unwrap().to_string(),
            datetime: records.get(1).unwrap().to_string(),
            location: records.get(2).unwrap().to_string(),
            end_date: records.get(3).unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct MenuRecord {
    item1: String,
    item2: String,
    item3: String,
}

impl MenuRecord {
    fn new(records: StringRecord) -> Self {
        Self {
            item1: records.get(0).unwrap().to_string(),
            item2: records.get(1).unwrap().to_string(),
            item3: records.get(2).unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ItemRecord {
    name: String,
    description: String,
    ingredients: String,
}

impl ItemRecord {
    fn new(records: StringRecord) -> Self {
        Self {
            name: records.get(0).unwrap().to_string(),
            description: records.get(1).unwrap().to_string(),
            ingredients: records.get(2).unwrap().to_string(),
        }
    }
}

fn read_csv<F, T>(filename: String, constructor: F) -> Result<Vec<T>>
where
    F: Fn(StringRecord) -> T,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => return Err(anyhow!("{:?}", err)),
    };
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_reader(transcoded);

    let vendors: Vec<_> = rdr
        .records()
        .map(|record_wrapped| match record_wrapped {
            Ok(record) => record,
            Err(_) => StringRecord::new(),
        })
        .map(|record| constructor(record))
        .collect();

    Ok(vendors)
}

async fn repopulate_database() -> Result<()> {
    let params: argon2::Params = argon2::Params::new(16, 1, 1, 32.into()).unwrap();
    // Setup our connection to the database
    let db: Surreal<Client> = server::app::db_connect().await.unwrap();

    // Delete all the vendors since we dont want our database getting massive
    // Add the throwaway arg so Rust can infer the typing of the deleted elements
    let _: Vec<Vendor> = db.delete("vendors").await?;

    let vendor_records = read_csv("./vendors.csv".into(), |v| VendorRecord::new(v)).unwrap();

    let vendors_vec: Vec<Vendor> = vendor_records
        .iter()
        .map(|vendor_record| {
            let mut vendor: Vendor = Vendor::new(vendor_record.name.clone());
            vendor.phone_number = Cow::Owned(vendor_record.phone.clone());
            vendor.email = Cow::Owned(vendor_record.email.clone());
            vendor
        })
        .collect();

    for vendor in vendors_vec {
        let _: Vendor = db
            .create(("vendors", vendor.uuid.to_string()))
            .content(vendor)
            .await
            .unwrap()
            .unwrap();
    }

    println!("Finished vendors");

    let _: Vec<Event> = db.delete("events").await?;

    let event_records = read_csv("./events.csv".into(), |e| EventRecord::new(e)).unwrap();

    /*let address = location.clone();
    // TODO: See about having a static instance of this
    // PERF: Check on custom endpoint
    let osm = Openstreetmap::new();
    let result = osm.forward(&address);
    let cords = match result {
        Ok(point_vec) => point_vec[0],
        Err(err) => {
            warn!("{}", err);
            Point::new(0.0, 0.0)
        }
    };*/
    let events_vec: Vec<Event> = event_records
        .iter()
        .map(|event_record| {
            let mut event: Event = Event::new(
                event_record.datetime.clone(),
                event_record.location.clone(),
                None,
            );
            event.repeat_end = Cow::Owned(event_record.end_date.clone());
            event.name = Cow::Owned(event_record.name.clone());
            event
        })
        .collect();

    for event in events_vec {
        let _: Event = db
            .create(("events", event.uuid.to_string()))
            .content(event)
            .await
            .unwrap()
            .unwrap();
    }

    let _: Vec<Item> = db.delete("items").await?;

    let item_records = read_csv("./food.csv".into(), |i| ItemRecord::new(i)).unwrap();

    let items_vec: Vec<Item> = item_records
        .iter()
        .map(|item_record| {
            let mut item: Item = Item::new(item_record.name.clone(), None);
            item.description = Cow::Owned(item_record.description.clone());
            item
        })
        .collect();

    for item in items_vec {
        let _: Item = db
            .create(("items", item.uuid.to_string()))
            .content(item)
            .await
            .unwrap()
            .unwrap();
    }

    let item_vec: Vec<Item> = db.select("items").await?;

    let _: Vec<Menu> = db.delete("menus").await?;

    let menu_records = read_csv("./menus.csv".into(), |m| MenuRecord::new(m)).unwrap();

    let items_list: Vec<Item> = db.select("items").await.unwrap();
    let menus_vec: Vec<Menu> = menu_records
        .iter()
        .map(|menu_record| {
            let mut menu: Menu = Menu::new(None);
            menu.name = format!(
                "{}:{}:{}",
                menu_record.item1.clone(),
                menu_record.item2.clone(),
                menu_record.item3.clone()
            )
            .into();
            let things_list: Vec<Thing> = items_list
                .clone()
                .iter()
                .map(|item| item.clone())
                .filter(|item| {
                    menu_record.item1 == item.name
                        || menu_record.item2 == item.name
                        || menu_record.item3 == item.name
                })
                .map(|item| Thing {
                    tb: "items".into(),
                    id: Id::String(item.uuid.into()),
                })
                .collect();
            menu.items = things_list;
            menu
        })
        .collect();

    for menu in menus_vec {
        let _: Menu = db
            .create(("menus", menu.uuid.to_string()))
            .content(menu)
            .await
            .unwrap()
            .unwrap();
    }

    println!("Test");

    let sql = r#"
            FOR $vendor IN (SELECT VALUE id FROM vendors) {
                UPDATE $vendor SET events = (SELECT VALUE id FROM events ORDER BY rand() LIMIT 3);
                UPDATE $vendor SET menus = (SELECT VALUE id FROM menus ORDER BY rand() LIMIT 2);
                FOR $menu IN (SELECT VALUE id FROM $vendor.menus) {
                    FOR $item IN (SELECT VALUE id FROM $menu.items) {
                        UPDATE $item SET vendor = $vendor;
                    };
                    UPDATE $menu SET vendor = $vendor;
                };
                FOR $event IN (SELECT VALUE id FROM $vendor.events) {
                    UPDATE $event SET vendor = $vendor;
                    UPDATE $event SET menu = (SELECT VALUE id FROM $vendor.menus ORDER BY rand() LIMIT 1);
                };
            };
        "#;

    let mut res = db.query(sql).await?;

    Ok(())
}

async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/vendor/get?vendor_id=n2cfynuwl9s5y9967xnk")
        .await?
        .print()
        .await;

    hc.do_get("/vendor/get").await?.print().await;

    hc.do_get("/vendor/add?name=War Pig Smokehouse&auth_token=12345")
        .await?
        .print()
        .await;

    hc.do_get("/vendor/remove?vendor_id=War Pig Smokehouse")
        .await?
        .print()
        .await;

    hc.do_get("/event/get?event_id=2ghydtjsubk8ip9f4mzd")
        .await?
        .print()
        .await;

    let datetime =
        serde_json::to_string::<DateTime<Local>>(&Local::now()).unwrap_or("".to_string());
    let location = serde_json::to_string(&Location::new(50, 50)).unwrap_or("".to_string());
    let repetition = serde_json::to_string(&ReoccurancePattern::OneTime).unwrap_or("".to_string());

    hc.do_get(
        format!(
            "/event/add?datetime={0}&location={1}&repetition={2}&vendor_id=n2cfynuwl9s5y9967xnk",
            datetime, location, repetition
        )
        .as_str(),
    )
    .await?
    .print()
    .await;

    hc.do_get("/event/remove?event_id=123").await?.print().await;

    hc.do_get("/menu/get?menu_id=z3vor00unpw3to3synfm")
        .await?
        .print()
        .await;

    hc.do_get("/menu/add?name=name&vendor_id=123")
        .await?
        .print()
        .await;

    hc.do_get("/menu/remove?menu_id=123").await?.print().await;

    hc.do_get("/item/get?item_id=jswl4z6ynnaai2n4kdhp")
        .await?
        .print()
        .await;

    hc.do_get("/item/add?name=name&vendor_id=123")
        .await?
        .print()
        .await;

    hc.do_get("/item/remove?item_id=123").await?.print().await;
    Ok(())
}

async fn check_route(route: &str) -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let resp = hc.do_get(route.into()).await?.json_body();
    assert!(resp.is_ok());
    assert!(resp.unwrap().to_string().len() > 2);
    Ok(())
}

async fn handlers_test() -> Result<()> {
    // Vendor auth_token setup
    /*let routes = vec![
        "/v1/vendors",
        "/v1/events",
        "/v1/vendors/088ADD402AC44769A6A725FD3225A4A1",
        "/v1/vendors/9C6569119B4046D2A3F584ACD20D4DA9/events",
    ];

    for route in routes {
        let result = check_route(route).await;
        match result {
            Ok(()) => println!("->> {:<60} - {}", route, "PASSED".green().underline()),
            Err(err) => println!(
                "->> {:<60} - {} - {:?}",
                route,
                "FAILED".red().underline(),
                err
            ),
        };
    }*/

    Ok(())
}
