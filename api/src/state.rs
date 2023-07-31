use axum::extract::FromRef;

use crate::{models::AppConfig, services::UserService};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: UserService
}