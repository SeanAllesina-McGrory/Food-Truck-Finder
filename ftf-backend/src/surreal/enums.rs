pub enum Day {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

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

pub enum ReoccurancePattern {
    OneTime,
    Daily,
    Weekly { days: Vec<Day>, spacing: u32 },
    Monthly { day_of_month: u32, spacing: u32 },
    Yearly { month: Month, day_of_month: u32 },
}
