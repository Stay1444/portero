use std::fmt::{write, Debug};

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateRange {
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeRange {
    pub begin: u8,
    pub end: u8
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub date_range: DateRange,
    pub time_range: TimeRange,
    pub weekdays: Vec<Weekday>
}

impl Schedule {
    pub fn contains(&self, time: DateTime<Utc>) -> bool {
        todo!()
    }
}