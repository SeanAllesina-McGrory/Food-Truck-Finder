#![allow(unused)]
#[path = "../src/server.rs"]
mod server;
#[path = "../src/database.rs"]
mod database;
use crate::database::models::{ReoccurancePattern, Vendor, Item};
use anyhow::Result;
use chrono::prelude::*;
use geoutils::Location;
use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[tokio::test]
async fn database_dev() -> Result<()> {
    let db: Surreal<Client> = server::app::db_connect().await.unwrap();
    
    let vendor1 = Vendor::new(String::from("Song2"),String::from("1111"));

    println!("Hello, Cruel World!");

    let vendor_db: Vendor = db.create(("vendor", vendor1.uuid.to_string().clone())).content(vendor1.clone()).await?.unwrap();

    println!("{:?}", vendor1);

    let vendor: Vec<Vendor> = match db.select("vendor").await {
        Ok(vendor) => vendor,
        Err(err) => {
            println!("Inside the err");
            println!("{:?}", err); 
            return Ok(());
        }
    };

    println!("Hello");

    vendor.iter().filter(|v| {
        v.name == "Song2"
    }).for_each(|v| {
        println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{:?}\n{:?}\n",
                 v.uuid,
                 v.name,
                 v.auth_token,
                 v.description,
                 v.vendor_type,
                 v.email,
                 v.phone_number,
                 v.website,
                 v.events,
                 v.menu);
    });

    let item = Item::new(String::from("Taco"), vendor1);

    println!("Hello world");

    println!("{}", item);

    let item: Item = db.create(("item", item.uuid.to_string().clone())).content(item).await?.unwrap();


    Ok(())
}

#[tokio::test]
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
    let repetition = serde_json::to_string(&ReoccurancePattern::OneTime)
        .unwrap_or("".to_string());

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
