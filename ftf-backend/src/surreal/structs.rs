use serde::{Serialize, Deserialize};
use chrono::naive::NaiveDateTime;
use geoutils::Location;
use std::borrow::Cow;
use crate::surreal::enums::ReoccurancePattern;

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    name: Cow<'static, str>,
    auth_token: Cow<'static, str>,
    description: Cow<'static, str>,
    vendor_type: Cow<'static, str>,
    email: Cow<'static, str>,
    phone_number: Cow<'static, str>,
    website: Cow<'static, str>,
    events: Vec<Event>,
    menus: Vec<Menu>,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    datetime: Cow<'static, NaiveDateTime>,
    location: Cow<'static, str>,
    menu: Menu, 
    //repeat_schedule: Cow<'static, ReoccurancePattern>, 
    repeat_end: Cow<'static, NaiveDateTime>,
    vendor: Vendor,
}
    
#[derive(Serialize, Deserialize)]
pub struct Menu {
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    price: u32,
    ingredients: Cow<'static, str>,
    picture: Cow<'static, str>,
    vendor: Vendor,
}
