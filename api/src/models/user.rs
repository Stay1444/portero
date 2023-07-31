use serde::{Serialize, Deserialize};
use sqlx::types::Json;

use super::time::Schedule;
use crate::{error::Result, Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserForDatabase {
    pub id: i64,
    pub name: String,
    pub code: String,

    pub schedule: Option<sqlx::types::JsonValue>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User  {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub schedule: Option<Schedule>
}

impl User {
    pub fn from_db(user: UserForDatabase) -> Result<User> {
        let schedule: Option<Schedule> = match user.schedule {
            Some(schedule) => {
                <Option<Schedule> as Deserialize>::deserialize(schedule)
                    .map_err(|_| Error::DeserializationFail)?
            },
            None => None
        };

        Ok(User {
            id: user.id,
            name: user.name,
            code: user.code,
            schedule
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserForUpdate {
    pub name: Option<String>,
    pub code: Option<String>,

    pub schedule: Option<Schedule>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserForCreate {
    pub name: String,
    pub schedule: Option<Schedule>
}