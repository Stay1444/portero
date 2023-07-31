use std::fmt::Debug;

use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub mod user;
pub mod time;

#[derive(Serialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T
}

#[derive(Clone)]
pub struct AppConfig {
    pub admin_password: String
}