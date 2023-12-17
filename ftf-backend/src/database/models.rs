use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use surrealdb::sql::Thing;
use uuid::Uuid;

// TODO: Create multiple dispatch constructors for structs here
// Or the rust equivalent of multiple dispatch since that isnt possible

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vendor {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub auth_token: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub vendor_type: Cow<'static, str>,
    pub email: Cow<'static, str>,
    pub phone_number: Cow<'static, str>,
    pub website: Cow<'static, str>,
    pub events: Vec<Thing>,
    pub menus: Vec<Thing>,
    pub items: Vec<Thing>,
}

impl Vendor {
    pub fn new(name: String, auth_token: String) -> Self {
        Vendor {
            uuid: Cow::Owned(String::from(
                Uuid::new_v4()
                    .simple()
                    .encode_upper(&mut uuid::Uuid::encode_buffer()),
            )),
            name: name.into(),
            auth_token: auth_token.into(),
            description: String::from("").into(),
            vendor_type: String::from("").into(),
            email: String::from("").into(),
            phone_number: String::from("").into(),
            website: String::from("").into(),
            events: vec![],
            menus: vec![],
            items: vec![],
        }
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID: {}\nName: {}\nAuth Token: {}\nDescription: {}\nVendor Type: {}\nEmail: {}\nPhone Number: {}\nWebsite: {}\nEvents: {:?}\nMenus: {:?}\nItems: {:?}", self.uuid, self.name, self.auth_token, self.description, self.vendor_type, self.email, self.phone_number, self.website, self.events, self.menus, self.items)
    }
}

impl Into<Cow<'static, Vendor>> for Vendor {
    fn into(self) -> Cow<'static, Vendor> {
        Cow::Owned(self)
    }
}

impl From<serde_json::Value> for Vendor {
    fn from(value: serde_json::Value) -> Self {
        let name = match value.get("name") {
            Some(name) => match name.as_str() {
                Some(name) => name.to_owned(),
                None => return Vendor::default(),
            },
            None => return Vendor::default(),
        };
        let auth_token = match value.get("auth_token") {
            Some(auth_token) => match auth_token.as_str() {
                Some(auth_token) => auth_token.to_owned(),
                None => return Vendor::default(),
            },
            None => return Vendor::default(),
        };

        let mut vendor = Vendor::new(name.to_string(), auth_token.to_string());
        if let Some(description) = value.get("description") {
            match description.as_str() {
                Some(description) => vendor.description = description.to_owned().into(),
                None => (),
            };
        }
        if let Some(vendor_type) = value.get("vendor_type") {
            match vendor_type.as_str() {
                Some(vendor_type) => vendor.vendor_type = vendor_type.to_owned().into(),
                None => (),
            };
        }
        if let Some(email) = value.get("email") {
            match email.as_str() {
                Some(email) => vendor.email = email.to_owned().into(),
                None => (),
            };
        }
        if let Some(phone_number) = value.get("phone_number") {
            match phone_number.as_str() {
                Some(phone_number) => vendor.phone_number = phone_number.to_owned().into(),
                None => (),
            };
        }
        if let Some(website) = value.get("website") {
            match website.as_str() {
                Some(website) => vendor.website = website.to_owned().into(),
                None => (),
            };
        }
        vendor
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
            items: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub datetime: Cow<'static, str>,
    pub location: Cow<'static, str>,
    pub menu: Option<Thing>,
    pub repeat_schedule: Cow<'static, ReoccurancePattern>,
    pub repeat_end: Cow<'static, str>,
    pub vendor: Option<Thing>,
}

impl Event {
    pub fn new(datetime: String, location: String, vendor: Option<Thing>) -> Self {
        Event {
            uuid: Cow::Owned(String::from(
                Uuid::new_v4()
                    .simple()
                    .encode_upper(&mut uuid::Uuid::encode_buffer()),
            )),
            name: "".into(),
            datetime: datetime.clone().into(),
            location: location.into(),
            menu: None,
            repeat_schedule: ReoccurancePattern::OneTime.into(),
            repeat_end: datetime.into(),
            vendor: vendor.into(),
        }
    }

    pub fn with_vendor(mut self, vendor_id: String) -> Self {
        self.vendor = Some(Thing {
            tb: "vendors".into(),
            id: vendor_id.into(),
        });
        self
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID: {}\nName: {}\nDateTime: {}\nLocation: {}\nMenu: {:?}\nRepeats: {}\nEnds: {}\nVendor: {}", self.uuid, self.name, self.datetime, self.location, self.menu, self.repeat_schedule, self.repeat_end, match &self.vendor {
        Some(thing) => thing.id.to_string(),
        None => "".into(),
        })
    }
}

impl From<serde_json::Value> for Event {
    fn from(value: serde_json::Value) -> Self {
        let datetime = match value.get("datetime") {
            Some(datetime) => match datetime.as_str() {
                Some(datetime) => datetime.to_owned(),
                None => return Event::default(),
            },
            None => return Event::default(),
        };
        let location = match value.get("location") {
            Some(location) => match location.as_str() {
                Some(location) => location.to_owned(),
                None => return Event::default(),
            },
            None => return Event::default(),
        };
        let mut event = Event::new(datetime, location, None);

        if let Some(vendor_id) = value.get("vendor_id") {
            event.vendor = Some(Thing {
                tb: "vendors".into(),
                id: vendor_id.to_string().into(),
            });
        }
        if let Some(name) = value.get("name") {
            match name.as_str() {
                Some(name) => event.name = name.to_owned().into(),
                None => (),
            };
        }
        if let Some(menu) = value.get("menu") {
            match menu.as_str() {
                Some(menu) => Some(Thing {
                    tb: "menu".into(),
                    id: menu.into(),
                }),
                None => None,
            };
        }
        if let Some(repeat_schedule) = value.get("repeat_schedule") {
            event.repeat_schedule = ReoccurancePattern::from(repeat_schedule.to_owned()).into()
        }
        if let Some(repeat_end) = value.get("repeat_end") {
            match repeat_end.as_str() {
                Some(repeat_end) => event.repeat_end = repeat_end.to_owned().into(),
                None => event.repeat_end = event.datetime.clone().to_owned().into(),
            };
        }
        event
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Menu {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub items: Vec<Thing>,
    pub vendor: Option<Thing>,
}

impl Menu {
    pub fn new(vendor: Option<Thing>) -> Self {
        Menu {
            uuid: Cow::Owned(String::from(
                Uuid::new_v4()
                    .simple()
                    .encode_upper(&mut uuid::Uuid::encode_buffer()),
            )),
            name: "".into(),
            items: Vec::new(),
            vendor: vendor.into(),
        }
    }

    pub fn with_vendor(mut self, vendor_id: String) -> Self {
        self.vendor = Some(Thing {
            tb: "vendors".into(),
            id: vendor_id.into(),
        });
        self
    }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UUID: {}\nName: {}\nItems: {:?}\nVendor: {}",
            self.uuid,
            self.name,
            self.items,
            match &self.vendor {
                Some(thing) => thing.id.to_string(),
                None => "".into(),
            },
        )
    }
}

impl From<serde_json::Value> for Menu {
    fn from(value: serde_json::Value) -> Self {
        let mut menu = Menu::new(None);

        if let Some(vendor_id) = value.get("vendor_id") {
            menu.vendor = Some(Thing {
                tb: "vendors".into(),
                id: vendor_id.to_string().into(),
            });
        }

        if let Some(name) = value.get("name") {
            match name.as_str() {
                Some(name) => menu.name = name.to_owned().into(),
                None => (),
            };
        }

        if let Some(items) = value.get("items") {
            match items.as_array() {
                Some(items) => {
                    menu.items = items
                        .iter()
                        .map(|i| i.clone())
                        .map(|item| Thing {
                            tb: "items".into(),
                            id: match item.as_str() {
                                Some(item_id) => item_id.into(),
                                None => "".into(),
                            },
                        })
                        .filter(|thing| thing.id != "".into())
                        .collect()
                }
                None => (),
            }
        }

        menu
    }
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            uuid: "".into(),
            name: "".into(),
            items: Vec::new(),
            vendor: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub uuid: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub price: Cow<'static, str>,
    pub picture: Cow<'static, str>,
    pub vendor: Option<Thing>,
}

impl Item {
    pub fn new(name: String, vendor: Option<Thing>) -> Self {
        Item {
            uuid: Cow::Owned(String::from(
                Uuid::new_v4()
                    .simple()
                    .encode_upper(&mut uuid::Uuid::encode_buffer()),
            )),
            name: name.into(),
            description: "".into(),
            price: "".into(),
            picture: "".into(),
            vendor: vendor.into(),
        }
    }

    pub fn with_vendor(mut self, vendor_id: String) -> Self {
        self.vendor = Some(Thing {
            tb: "vendors".into(),
            id: vendor_id.into(),
        });
        self
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

impl From<serde_json::Value> for Item {
    fn from(value: serde_json::Value) -> Self {
        let name = match value.get("name") {
            Some(name) => match name.as_str() {
                Some(name) => name.to_owned(),
                None => return Item::default(),
            },
            None => return Item::default(),
        };

        let mut item = Item::new(name, None);

        if let Some(vendor_id) = value.get("vendor_id") {
            item.vendor = Some(Thing {
                tb: "vendors".into(),
                id: vendor_id.to_string().into(),
            });
        }

        item
    }
}

impl Default for Item {
    fn default() -> Self {
        Item {
            uuid: "".into(),
            name: "".into(),
            description: "".into(),
            price: "".into(),
            picture: "".into(),
            vendor: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Day {
    NONE,
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

impl From<serde_json::Value> for Day {
    fn from(value: serde_json::Value) -> Self {
        let day = match value.as_str() {
            Some(day) => day,
            None => return Day::NONE,
        };
        let day = day.to_uppercase();
        match day.as_str() {
            "MONDAY" => Day::MONDAY,
            "TUESDAY" => Day::TUESDAY,
            "WEDNESDAY" => Day::WEDNESDAY,
            "THURSDAY" => Day::THURSDAY,
            "FRIDAY" => Day::FRIDAY,
            "SATURDAY" => Day::SATURDAY,
            "SUNDAY" => Day::SUNDAY,
            _ => Day::NONE,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Month {
    None,
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

impl From<serde_json::Value> for Month {
    fn from(value: serde_json::Value) -> Self {
        let month = match value.as_str() {
            Some(month) => month,
            None => return Month::None,
        };
        let month = month.to_uppercase();
        match month.as_str() {
            "JANUARY" => Month::January,
            "FEBUARY" => Month::February,
            "MARCH" => Month::March,
            "APRIL" => Month::April,
            "MAY" => Month::May,
            "JUNE" => Month::June,
            "JULY" => Month::July,
            "AUGUST" => Month::August,
            "SEPTEMBER" => Month::September,
            "OCTOBER" => Month::October,
            "NOVEMBER" => Month::November,
            "DECEMBER" => Month::December,
            _ => Month::None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

impl From<serde_json::Value> for ReoccurancePattern {
    fn from(json: serde_json::Value) -> Self {
        let pattern = match json.get("pattern") {
            Some(pattern) => pattern,
            None => return ReoccurancePattern::None,
        };
        let pattern = match pattern.as_str() {
            Some(pattern) => pattern,
            None => return ReoccurancePattern::None,
        };
        match pattern {
            "OneTime" => ReoccurancePattern::OneTime,
            "Daily" => ReoccurancePattern::Daily,
            "Weekly" => {
                let days = match json.get("days") {
                    Some(days) => match days.as_array() {
                        Some(days) => days.iter().map(|day| day.to_owned().into()).collect(),
                        None => return ReoccurancePattern::None,
                    },
                    None => return ReoccurancePattern::None,
                };
                let spacing: u32 = match json.get("spacing") {
                    Some(spacing) => match spacing.as_u64() {
                        Some(spacing) => match spacing.try_into() {
                            Ok(spacing) => spacing,
                            Err(err) => {
                                println!("Unable to convert u64 to u32: {err:?}");
                                return ReoccurancePattern::None;
                            }
                        },
                        None => return ReoccurancePattern::None,
                    },
                    None => return ReoccurancePattern::None,
                };
                ReoccurancePattern::Weekly { days, spacing }
            }
            "Monthly" => {
                let day_of_month: u32 = match json.get("day_of_month") {
                    Some(day_of_month) => match day_of_month.as_u64() {
                        Some(day_of_month) => match day_of_month.try_into() {
                            Ok(day_of_month) => day_of_month,
                            Err(err) => {
                                println!("Unable to convert u64 to u32: {err:?}");
                                return ReoccurancePattern::None;
                            }
                        },
                        None => return ReoccurancePattern::None,
                    },
                    None => return ReoccurancePattern::None,
                };
                let spacing: u32 = match json.get("spacing") {
                    Some(spacing) => match spacing.as_u64() {
                        Some(spacing) => match spacing.try_into() {
                            Ok(spacing) => spacing,
                            Err(err) => {
                                println!("Unable to convert u64 to u32: {err:?}");
                                return ReoccurancePattern::None;
                            }
                        },
                        None => return ReoccurancePattern::None,
                    },
                    None => return ReoccurancePattern::None,
                };
                ReoccurancePattern::Monthly {
                    day_of_month,
                    spacing,
                }
            }
            "Yearly" => {
                let month = match json.get("month") {
                    Some(month) => month.to_owned().into(),
                    None => return ReoccurancePattern::None,
                };
                let day_of_month: u32 = match json.get("day_of_month") {
                    Some(day_of_month) => match day_of_month.as_u64() {
                        Some(day_of_month) => match day_of_month.try_into() {
                            Ok(day_of_month) => day_of_month,
                            Err(err) => {
                                println!("Unable to convert u64 to u32: {err:?}");
                                return ReoccurancePattern::None;
                            }
                        },
                        None => return ReoccurancePattern::None,
                    },
                    None => return ReoccurancePattern::None,
                };
                ReoccurancePattern::Yearly {
                    month,
                    day_of_month,
                }
            }
            _ => ReoccurancePattern::None,
        }
    }
}
// endregion:   -- Data Structures
