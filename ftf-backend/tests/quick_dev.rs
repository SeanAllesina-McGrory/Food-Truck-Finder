#![allow(unused)]
#[path = "../src/database.rs"]
mod database;
#[path = "../src/server.rs"]
mod server;
use crate::database::models::{Event, Item, Menu, ReoccurancePattern, Vendor};
use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::prelude::*;
use csv::{ReaderBuilder, StringRecord};
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use geoutils::Location;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{fs::File, io::Read};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

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
        .delimiter(b',')
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
            let salt = SaltString::generate(&mut OsRng);

            let argon2 = Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                params.clone(),
            );

            let password_hash = argon2
                .hash_password(vendor_record.name.clone().as_bytes(), &salt)
                .unwrap()
                .to_string();

            let mut vendor: Vendor = Vendor::new(vendor_record.name.clone(), password_hash);
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

    let vendors: Vec<Vendor> = db.select("vendors").await?;

    /*vendors.iter().for_each(|vendor| {
        println!("{}\n", vendor);
    });*/

    let mut vendors = vendors.into_iter();

    let _: Vec<Event> = db.delete("events").await?;

    let event_records = read_csv("./events.csv".into(), |e| EventRecord::new(e)).unwrap();

    let events_vec: Vec<Event> = event_records
        .iter()
        .map(|event_record| {
            let mut event: Event = Event::new(
                event_record.datetime.clone(),
                event_record.location.clone(),
                vendors.next().unwrap(),
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

    let events: Vec<Event> = db.select("events").await?;

    vendors = db.select("vendors").await?.into_iter();

    /*events.iter().for_each(|event| {
        println!("{:?}\n", event);
    });*/

    let _: Vec<Menu> = db.delete("menus").await?;

    let menu_records = read_csv("./menus.csv".into(), |m| MenuRecord::new(m)).unwrap();

    let menus_vec: Vec<Menu> = menu_records
        .iter()
        .map(|menu_record| {
            let mut menu: Menu = Menu::new(
                format!(
                    "{}:{}:{}",
                    menu_record.item1.clone(),
                    menu_record.item2.clone(),
                    menu_record.item3.clone()
                ),
                vendors.next().unwrap(),
            );
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

    let menus: Vec<Menu> = db.select("menus").await?;

    vendors = db.select("vendors").await?.into_iter();

    /*menus.iter().for_each(|menu| {
        println!("{:?}\n", menu);
    });*/

    let _: Vec<Item> = db.delete("items").await?;

    let item_records = read_csv("./food.csv".into(), |i| ItemRecord::new(i)).unwrap();

    let items_vec: Vec<Item> = item_records
        .iter()
        .map(|item_record| {
            let mut item: Item = Item::new(item_record.name.clone(), vendors.next().unwrap());
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

    let items: Vec<Item> = db.select("items").await?;

    /*items.iter().for_each(|item| {
        println!("{:?}Test\n", item);
    });*/

    Ok(())
}

#[tokio::test]
async fn quick_dev() -> Result<()> {
    //repopulate_database().await?;
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

#[tokio::test]
async fn handlers_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Vendors
    hc.do_get("/vendor/get").await?.print().await;
    hc.do_get("/vendor/get?vendor_id=").await?.print().await;
    hc.do_get("/vendor/get?event_id=").await?.print().await;
    hc.do_get("/vendor/get?menu_id=").await?.print().await;
    hc.do_get("/vendor/get?item_id=").await?.print().await;
    Ok(())
}
