use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vendor {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub auth_token: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub vendor_type: Cow<'static, str>,
    pub email: Cow<'static, str>,
    pub phone_number: Cow<'static, str>,
    pub website: Cow<'static, str>,
    pub events: Vec<Event>,
    pub menus: Vec<Item>,
}

impl Vendor {
    pub fn new(name: String, auth_token: String) -> Self {
        Vendor {
            uuid: Cow::Owned(Uuid::new_v4().to_string().into()),
            name: name.into(),
            auth_token: auth_token.into(),
            description: String::from("").into(),
            vendor_type: String::from("").into(),
            email: String::from("").into(),
            phone_number: String::from("").into(),
            website: String::from("").into(),
            events: vec![],
            menus: vec![],
        }
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID: {}\nName: {}\nAuth Token: {}\nDescription: {}\nVendor Type: {}\nEmail: {}\nPhone Number: {}\nWebsite: {}\nEvents: {:?}\nMenus: {:?}", self.uuid, self.name, self.auth_token, self.description, self.vendor_type, self.email, self.phone_number, self.website, self.events, self.menus)
    }
}

impl Into<Cow<'static, Vendor>> for Vendor {
    fn into(self) -> Cow<'static, Vendor> {
        Cow::Owned(self)
    }
}

impl Default for Vendor {
    fn default() -> Self {
        Vendor {
            uuid: "".into(),
            name: "".into(),
            auth_token: "".into(),
            description: "".into(),
            vendor_type: "".into(),
            email: "".into(),
            phone_number: "".into(),
            website: "".into(),
            events: Vec::new(),
            menus: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VendorGetParams {
    pub vendor_id: Option<String>,
    pub event_id: Option<String>,
    pub menu_id: Option<String>,
    pub item_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VendorAddParams {
    pub name: String,
    pub auth_token: String,
    pub description: Option<String>,
    pub vendor_type: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub website: Option<String>,
    pub photo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VendorRemoveParams {
    pub vendor_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub datetime: Cow<'static, str>,
    pub location: Cow<'static, str>,
    pub menu: Option<Menu>,
    pub repeat_schedule: Cow<'static, ReoccurancePattern>,
    pub repeat_end: Cow<'static, str>,
    pub vendor: Option<Vendor>,
}

impl Event {
    pub fn new(datetime: String, location: String, vendor: Vendor) -> Self {
        Event {
            uuid: Cow::Owned(Uuid::new_v4().to_string().into()),
            name: "".into(),
            datetime: datetime.clone().into(),
            location: location.into(),
            menu: None,
            repeat_schedule: ReoccurancePattern::OneTime.into(),
            repeat_end: datetime.into(),
            vendor: vendor.into(),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID: {}\nName: {}\nDateTime: {}\nLocation: {}\nMenu: {:?}\nRepeats: {}\nEnds: {}\nVendor: {:?}", self.uuid, self.name, self.datetime, self.location, self.menu, self.repeat_schedule, self.repeat_end, self.vendor)
    }
}

impl Default for Event {
    fn default() -> Self {
        Event {
            uuid: "".into(),
            name: "".into(),
            datetime: "".into(),
            location: "".into(),
            menu: None,
            repeat_schedule: ReoccurancePattern::None.into(),
            repeat_end: "".into(),
            vendor: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventGetParams {
    pub event_id: Option<String>,
    pub vendor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EventAddParams {
    pub datetime: String,
    pub location: String,
    pub repetition: String,
    pub vendor_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EventRemoveParams {
    pub event_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Menu {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub items: Cow<'static, Vec<Item>>,
    pub vendor: Cow<'static, Vendor>, // FIX: Vendor is a Cow is some instances and a vendor in
                                      // others, Choose one
}

impl Menu {
    pub fn new(name: String, vendor: Vendor) -> Self {
        Menu {
            uuid: Cow::Owned(Uuid::new_v4().to_string().into()),
            name: name.into(),
            items: Cow::Owned(Vec::new()),
            vendor: vendor.into(),
        }
    }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UUID: {}\nName: {}\nItems: {:?}\nVendor: {}",
            self.uuid, self.name, self.items, self.vendor
        )
    }
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            uuid: "".into(),
            name: "".into(),
            items: Cow::Owned(Vec::new()),
            vendor: Vendor::default().into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MenuGetParams {
    pub menu_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MenuAddParams {
    pub name: String,
    pub items: Option<String>,
    pub vendor_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MenuRemoveParams {
    pub menu_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub price: Cow<'static, str>,
    pub picture: Cow<'static, str>,
    pub vendor: Vendor,
}

impl Item {
    pub fn new(name: String, vendor: Vendor) -> Self {
        Item {
            uuid: Cow::Owned(Uuid::new_v4().to_string().into()),
            name: name.into(),
            description: "".into(),
            price: "".into(),
            picture: "".into(),
            vendor: vendor.into(),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UUID: {}\nName: {}\nDescription: {}\nPrice: {}\nPicture: {}\nVendor: {:?}",
            self.uuid, self.name, self.description, self.price, self.picture, self.vendor
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct ItemGetParams {
    pub item_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemAddParams {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub picture: Option<String>,
    pub vendor_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemRemoveParams {
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Day {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ReoccurancePattern {
    None,
    OneTime,
    Daily,
    Weekly { days: Vec<Day>, spacing: u32 },
    Monthly { day_of_month: u32, spacing: u32 },
    Yearly { month: Month, day_of_month: u32 },
}

// region:      -- Data Structures
impl fmt::Display for ReoccurancePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<Cow<'static, ReoccurancePattern>> for ReoccurancePattern {
    fn into(self) -> Cow<'static, ReoccurancePattern> {
        Cow::Owned(self)
    }
}
// endregion:   -- Data Structures
