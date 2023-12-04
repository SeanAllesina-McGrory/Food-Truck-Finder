#![allow(unused)] // For development, remove before prod
use anyhow::{anyhow, Error, Result};
use std::collections::BTreeMap;
use std::env;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql;
use surrealdb::{Result as Surreal_Result, Surreal};

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    name: Cow<'static, str>,
    auth_token: Cow<'static, str>,
    description: Cow<'static, str>,
    vendor_type: Cow<'static, str>,
    email: Cow<'static, str>,
    phone_number: Cow<'static, str>,
    website: Cow<'static, str>,
}
// Test class to run a query on the database
// Will either be removed to turned private before prod
// TODO: Make this private
pub async fn query_db() -> Result<()> {
    // Create a mutable var so we can get the database out of the match statement
    let mut db;

    // Gets the db creds and returns them as a anyhow::Result<Vec<String>>
    match get_db_creds() {
        Ok(response) => {
            db = Surreal::new::<Ws>("localhost:8000").await?;
            db.signin(Root {
                username: &response[0],
                password: &response[1],
            })
            .await?;
        }
        Err(_) => return Err(anyhow!("Failed to connect to database")),
    }

    // Sets up the database info, will just use defaults if the env variables aren't set
    // Including this in the source shouldn't be a security risk since its just database names
    db.use_ns(match env::var("DBNS") {
        Ok(namespace) => namespace.to_owned(),
        Err(_) => "food_truck_finder".to_string(),
    })
    .use_db(match env::var("DBNM") {
        Ok(database_name) => database_name.to_owned(),
        Err(_) => "ftf_db".to_string(),
    })
    .await?;

    let vendor: Vec<Vendor> = db
        .create("vendor")
        .content(Vendor {
            name: "War Pig Smokehouse".into(),
            auth_token: "123".into(),
            description: "Brisket".into(),
            vendor_type: "Food Truck".into(),
            email: "email@email".into(),
            phone_number: "1234567890".into(),
            website: "123.com".into(),
        })
        .await?;

    let sql = r#"
        SELECT *
        FROM type::table($table)
    "#;

    let groups = db.query(sql).bind(("table", "vendor")).await?;

    println!("{:?}", groups);

    Ok(())
}

fn get_db_creds() -> Result<Vec<String>> {
    let username = match env::var("DBUN") {
        Ok(uname) => uname,
        Err(_) => {
            return Err(anyhow!(
                "Database username environment variable could not be resolved"
            ))
        }
    };
    let password = match env::var("DBPW") {
        Ok(pword) => pword,
        Err(_) => {
            return Err(anyhow!(
                "Database password environment variable could not be resolved"
            ))
        }
    };

    Ok(vec![username, password])
}
